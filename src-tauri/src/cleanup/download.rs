//! Downloads the cleanup model from its official source and checks it.
//!
//! The files come straight from Qwen's official Hugging Face repositories, so
//! the feature does not depend on anyone else's hosting. Every file is
//! verified against a pinned SHA-256 before it is used.

use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct Artifact {
    pub url: &'static str,
    pub file: &'static str,
    pub bytes: u64,
    pub sha256: &'static str,
}

/// Qwen2.5-1.5B-Instruct, 4-bit (Apache-2.0). https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF
pub const MODEL: Artifact = Artifact {
    url: "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf",
    file: "qwen2.5-1.5b-instruct-q4_k_m.gguf",
    bytes: 1_117_320_736,
    sha256: "6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e",
};

/// The matching tokenizer (Apache-2.0). https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct
pub const TOKENIZER: Artifact = Artifact {
    url: "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct/resolve/main/tokenizer.json",
    file: "tokenizer.json",
    bytes: 7_031_645,
    sha256: "c0382117ea329cdf097041132f6d735924b697924d6f6fc3945713e96ce87539",
};

pub fn is_present(dir: &Path, art: &Artifact) -> bool {
    fs::metadata(dir.join(art.file)).map(|m| m.len() == art.bytes).unwrap_or(false)
}

fn hash_file(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Downloads `art` into `dir` (resuming a partial file), verifies it, and
/// reports the number of bytes of this file received so far through `progress`.
pub async fn fetch(
    art: &Artifact,
    dir: &Path,
    cancel: &AtomicBool,
    mut progress: impl FnMut(u64),
) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("Could not create the folder: {e}"))?;
    let dest = dir.join(art.file);
    if is_present(dir, art) {
        progress(art.bytes);
        return Ok(());
    }
    let partial = dir.join(format!("{}.partial", art.file));
    let mut offset = fs::metadata(&partial).map(|m| m.len()).unwrap_or(0);
    if offset > art.bytes {
        let _ = fs::remove_file(&partial);
        offset = 0;
    }

    let client = reqwest::Client::builder()
        .user_agent("Quill")
        .build()
        .map_err(|e| e.to_string())?;
    let mut request = client.get(art.url);
    if offset > 0 {
        request = request.header("Range", format!("bytes={offset}-"));
    }
    let response = request.send().await.map_err(|e| format!("Could not connect: {e}"))?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("The download server answered {status}."));
    }
    // A server that ignores the range sends everything again from the start.
    if offset > 0 && status.as_u16() != 206 {
        offset = 0;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(offset == 0)
        .append(offset > 0)
        .open(&partial)
        .map_err(|e| format!("Could not write the file: {e}"))?;

    let mut received = offset;
    progress(received);
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        if cancel.load(Ordering::SeqCst) {
            return Err("Cancelled".into());
        }
        let chunk = chunk.map_err(|e| format!("The connection dropped: {e}"))?;
        file.write_all(&chunk).map_err(|e| format!("Could not write the file: {e}"))?;
        received += chunk.len() as u64;
        progress(received);
    }
    file.flush().map_err(|e| e.to_string())?;
    drop(file);

    if received != art.bytes {
        return Err(format!("The download was incomplete ({received} of {} bytes).", art.bytes));
    }
    let want = art.sha256.to_string();
    let check = partial.clone();
    let got = tokio::task::spawn_blocking(move || hash_file(&check))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| format!("Could not verify the file: {e}"))?;
    if got != want {
        let _ = fs::remove_file(&partial);
        return Err("The downloaded file did not match its checksum and was discarded. Please try again.".into());
    }
    fs::rename(&partial, &dest).map_err(|e| format!("Could not finish the file: {e}"))?;
    Ok(())
}
