//! "Remove filler words": an optional, fully on-device
//! cleanup of each dictation.
//!
//! After the speech model writes the words, a small text model removes
//! fillers, applies self-corrections ("three, no wait, four") and tidies
//! punctuation. Nothing leaves the computer.
//!
//! Rules this module keeps:
//! * **Off means off.** With the setting off, the model is never downloaded or
//!   loaded and uses no memory. Turning it off unloads it immediately.
//! * **Never worse than before.** If the model is not ready, runs out of time,
//!   or produces anything that fails the safety net in [`guard`], Quill pastes
//!   the original transcript.

mod download;
mod engine;
pub mod guard;
pub mod prompt;
mod qwen2q;

use crate::settings;
use engine::Engine;
use log::{debug, info, warn};
use serde::Serialize;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter};

/// The whole cleanup of one dictation must finish within this budget.
const BUDGET: Duration = Duration::from_secs(6);
/// The model is unloaded after this long without a dictation.
const IDLE_UNLOAD: Duration = Duration::from_secs(10 * 60);
/// Transcripts shorter than this are left to the simple built-in filler removal.
const MIN_WORDS: usize = 4;
/// Long dictations are cleaned a few sentences at a time.
const CHUNK_CHARS: usize = 360;

#[derive(Clone, Debug, Serialize, specta::Type)]
pub struct CleanupStatus {
    /// The setting in General.
    pub enabled: bool,
    /// Both model files are on disk and verified.
    pub files_ready: bool,
    pub downloading: bool,
    /// The model is in memory right now.
    pub loaded: bool,
    /// Bytes received so far while downloading (as a number the UI can use).
    pub downloaded_bytes: f64,
    pub total_bytes: f64,
    /// A plain-language problem to show, if the last download failed.
    pub error: Option<String>,
}

#[derive(Default)]
struct DownloadState {
    active: bool,
    downloaded: u64,
    error: Option<String>,
}

pub struct CleanupManager {
    app: AppHandle,
    dir: PathBuf,
    engine: Mutex<Option<Arc<Engine>>>,
    loading: AtomicBool,
    last_used: Mutex<Instant>,
    download: Mutex<DownloadState>,
    cancel: AtomicBool,
}

impl CleanupManager {
    pub fn new(app: &AppHandle) -> Result<Arc<Self>, String> {
        let dir = crate::portable::app_data_dir(app)
            .map_err(|e| format!("app data dir: {e}"))?
            .join("cleanup");
        let manager = Arc::new(Self {
            app: app.clone(),
            dir,
            engine: Mutex::new(None),
            loading: AtomicBool::new(false),
            last_used: Mutex::new(Instant::now()),
            download: Mutex::new(DownloadState::default()),
            cancel: AtomicBool::new(false),
        });
        // Free the memory after a quiet spell.
        let weak = Arc::downgrade(&manager);
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                let Some(m) = weak.upgrade() else { break };
                let idle = m.last_used.lock().map(|t| t.elapsed()).unwrap_or_default();
                if idle > IDLE_UNLOAD && m.is_loaded() {
                    info!("Cleanup model unloaded after {:?} idle", idle);
                    m.unload();
                }
            }
        });
        Ok(manager)
    }

    fn enabled(&self) -> bool {
        settings::get_settings(&self.app).smart_cleanup_enabled
    }

    fn model_dir(&self) -> &PathBuf {
        &self.dir
    }

    pub fn files_ready(&self) -> bool {
        download::is_present(self.model_dir(), &download::MODEL) && download::is_present(self.model_dir(), &download::TOKENIZER)
    }

    fn is_loaded(&self) -> bool {
        self.engine.lock().map(|e| e.is_some()).unwrap_or(false)
    }

    pub fn status(&self) -> CleanupStatus {
        let dl = self.download.lock().unwrap();
        CleanupStatus {
            enabled: self.enabled(),
            files_ready: self.files_ready(),
            downloading: dl.active,
            loaded: self.is_loaded(),
            downloaded_bytes: dl.downloaded as f64,
            total_bytes: (download::MODEL.bytes + download::TOKENIZER.bytes) as f64,
            error: dl.error.clone(),
        }
    }

    fn emit_status(&self) {
        let _ = self.app.emit("cleanup-status-changed", self.status());
    }

    /// Loads the model in the background (only if the setting is on and the
    /// files are present). Called when a recording starts, so it is usually
    /// ready by the time you stop speaking.
    pub fn preload(self: &Arc<Self>) {
        if !self.enabled() || !self.files_ready() || self.is_loaded() {
            return;
        }
        if self.loading.swap(true, Ordering::SeqCst) {
            return;
        }
        let this = self.clone();
        std::thread::spawn(move || {
            let started = Instant::now();
            let result = Engine::load(
                &this.dir.join(download::MODEL.file),
                &this.dir.join(download::TOKENIZER.file),
            );
            match result {
                Ok(engine) => {
                    info!(
                        "Cleanup model loaded in {:.2}s ({})",
                        started.elapsed().as_secs_f64(),
                        if engine.uses_gpu() { "graphics chip" } else { "CPU" }
                    );
                    // The setting may have been switched off while loading.
                    if this.enabled() {
                        *this.engine.lock().unwrap() = Some(Arc::new(engine));
                        *this.last_used.lock().unwrap() = Instant::now();
                    }
                }
                Err(e) => warn!("Cleanup model could not be loaded: {e:#}"),
            }
            this.loading.store(false, Ordering::SeqCst);
            this.emit_status();
        });
    }

    pub fn unload(&self) {
        if self.engine.lock().unwrap().take().is_some() {
            debug!("Cleanup model unloaded");
        }
        self.emit_status();
    }

    /// Cleans one dictation. `None` means "paste the original".
    pub async fn clean(self: &Arc<Self>, text: &str) -> Option<String> {
        if !self.enabled() || text.split_whitespace().count() < MIN_WORDS {
            return None;
        }
        let engine = self.engine.lock().unwrap().clone();
        let Some(engine) = engine else {
            // Not in memory yet: start loading for next time, paste this one as is.
            self.preload();
            debug!("Cleanup model not ready yet; pasting the plain transcript");
            return None;
        };
        let text = text.to_string();
        let started = Instant::now();
        let deadline = started + BUDGET;
        let result = tokio::task::spawn_blocking(move || clean_chunks(&engine, &text, deadline))
            .await
            .ok()
            .flatten();
        *self.last_used.lock().unwrap() = Instant::now();
        match &result {
            Some(_) => debug!("Cleanup finished in {:.2}s", started.elapsed().as_secs_f64()),
            None => debug!("Cleanup skipped or rejected after {:.2}s", started.elapsed().as_secs_f64()),
        }
        result
    }

    // ---------------------------------------------------------------- download

    pub fn start_download(self: &Arc<Self>) {
        {
            let mut dl = self.download.lock().unwrap();
            if dl.active || self.files_ready() {
                return;
            }
            dl.active = true;
            dl.error = None;
            dl.downloaded = 0;
        }
        self.cancel.store(false, Ordering::SeqCst);
        self.emit_status();

        let this = self.clone();
        tauri::async_runtime::spawn(async move {
            let model_bytes = download::MODEL.bytes;
            let mut last_emit = Instant::now() - Duration::from_secs(1);
            let result = async {
                let t = this.clone();
                download::fetch(&download::MODEL, &this.dir, &this.cancel, |n| {
                    t.download.lock().unwrap().downloaded = n;
                    if last_emit.elapsed() > Duration::from_millis(250) {
                        last_emit = Instant::now();
                        t.emit_status();
                    }
                })
                .await?;
                let t = this.clone();
                download::fetch(&download::TOKENIZER, &this.dir, &this.cancel, |n| {
                    t.download.lock().unwrap().downloaded = model_bytes + n;
                })
                .await
            }
            .await;
            {
                let mut dl = this.download.lock().unwrap();
                dl.active = false;
                match result {
                    Ok(()) => dl.error = None,
                    Err(e) if e == "Cancelled" => dl.error = None,
                    Err(e) => {
                        warn!("Cleanup model download failed: {e}");
                        dl.error = Some(e);
                    }
                }
            }
            this.emit_status();
            if this.files_ready() {
                info!("Cleanup model downloaded and verified");
                this.preload();
            }
        });
    }

    pub fn cancel_download(&self) {
        self.cancel.store(true, Ordering::SeqCst);
    }

    /// Deletes the model files to free the disk space.
    pub fn delete_files(&self) -> Result<(), String> {
        self.cancel_download();
        self.unload();
        if self.dir.exists() {
            std::fs::remove_dir_all(&self.dir).map_err(|e| format!("Could not delete the files: {e}"))?;
        }
        self.download.lock().unwrap().downloaded = 0;
        self.emit_status();
        Ok(())
    }
}

/// Splits long text at sentence ends into chunks of about `CHUNK_CHARS`.
fn split_chunks(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for sentence in text.split_inclusive(|c| matches!(c, '.' | '?' | '!')) {
        if !current.is_empty() && current.len() + sentence.len() > CHUNK_CHARS {
            chunks.push(std::mem::take(&mut current));
        }
        current.push_str(sentence);
    }
    if !current.trim().is_empty() {
        chunks.push(current);
    }
    chunks
}

/// Cleans each chunk and keeps a chunk's original text if its cleanup fails the safety net.
fn clean_chunks(engine: &Engine, text: &str, deadline: Instant) -> Option<String> {
    let mut parts = Vec::new();
    for chunk in split_chunks(text) {
        let piece = chunk.trim();
        if piece.is_empty() {
            continue;
        }
        match engine.clean(piece, deadline) {
            Ok(Some(out)) if guard::accept(piece, &out) => parts.push(out),
            Ok(Some(_)) => {
                debug!("Cleanup output rejected by the safety net; keeping the original text");
                parts.push(piece.to_string());
            }
            Ok(None) => return None, // out of time or cut off: keep the whole transcript as is
            Err(e) => {
                warn!("Cleanup failed: {e:#}");
                return None;
            }
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::split_chunks;

    #[test]
    fn short_text_is_one_chunk() {
        assert_eq!(split_chunks("Hello there. How are you?").len(), 1);
    }

    #[test]
    fn long_text_is_split_at_sentence_ends() {
        let sentence = "This is a fairly ordinary sentence about the weekly plan. ";
        let text = sentence.repeat(20);
        let chunks = split_chunks(&text);
        assert!(chunks.len() > 2);
        assert!(chunks.iter().all(|c| c.trim_end().ends_with('.')));
        assert_eq!(chunks.concat(), text);
    }
}
