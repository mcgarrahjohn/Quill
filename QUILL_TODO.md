# Quill: what to set up before shipping

The code, names, colors and icons are Quill. These items need your own accounts or decisions.

## 1. Placeholders to replace
- ~~`OWNER/quill`~~ DONE: now `mcgarrahjohn/quill` everywhere (README, CONTRIBUTING, updater endpoint, workflows, About page, Homebrew cask).
- `quill.example`: your real website domain (docs, donate and troubleshooting links in the app).
- `com.quill.desktop`: the bundle identifier. Pick it once; changing it later moves users' settings folders.

## 2. Auto-update keys (important)
`src-tauri/tauri.conf.json` still holds the **original author's updater public key**. Generate your own and replace it, or auto-update will never verify your builds:
`bunx tauri signer generate -w ~/.tauri/quill.key`, then put the public key in `plugins.updater.pubkey` and add the private key and password to your CI secrets.

## 3. Code signing
Handy's Windows signing command was removed (it pointed to the author's Azure account). Add your own `signCommand`, and macOS signing/notarization secrets, in `src-tauri/tauri.conf.json` and `.github/workflows`.

## 4. Model hosting (the one thing that is still named "handy")
The speech-model catalog downloads weights from Hugging Face repos under the `handy-computer/` organization and from the `blob.handy.computer` mirror (see `src-tauri/src/catalog/catalog.json`, `src-tauri/src/catalog/mod.rs`, `src-tauri/src/managers/model.rs`). These are the author's hosted files, so they work today but are not yours. To become independent: re-host the files under your own Hugging Face organization or storage (`scripts/mirror_models.py` helps) and change the repository ids and `MIRRORS` constant.

## 5. Other items
- `src/content/release-notes/` still holds the original project's changelog text with the product name swapped. Replace it with Quill's own notes and reset the version in `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json` and `package.json`.
- Nix packaging files were removed (they pinned the old package names).
- Add your own copyright line to LICENSE below the original one. Keep the original line.
- `transcribe-cpp` (the speech engine) and forks `cjpais/hf-hub` and `cjpais/tao` are still fetched from their original sources. They carry no Handy name.

## 6. Every place that still points at a placeholder (replace when you have the real thing)
| Placeholder | What it should become | Where it shows up |
| --- | --- | --- |
| ~~`https://quill.example/donate`~~ | DONE: Donate now opens your Stripe Payment Link `https://buy.stripe.com/aFa7sFgeha0FaBDepn14400`. Optional later: recreate it with the **Donate** button type (Advanced options) for a `donate.stripe.com` address, then change the one URL in `AboutSettings.tsx`. | About page (`src/components/settings/about/AboutSettings.tsx`) |
| ~~`https://github.com/OWNER/quill`~~ | DONE: `github.com/mcgarrahjohn/quill` (works once the repo is public) | About page "View on GitHub" |
| `https://quill.example/docs/...` | your docs site | Secure Input warning, Linux setup guide (`SecureInputWarning.tsx`, `LinuxGuide.tsx`) |
| ~~`OWNER/quill` releases URL~~ | DONE: points at `mcgarrahjohn/quill` releases | `tauri.conf.json` updater endpoint, `portableInstaller.ts` |
| ~~`https://github.com/OWNER/quill` and `Quill/1.0 (+...)`~~ | DONE: your repo URL | Sent as the Referer and User-Agent to AI providers in `src-tauri/src/llm_client.rs` |
| `com.quill.desktop` | your reverse-domain bundle id | `tauri.conf.json`, docs, README paths |
| `contact@quill.example` | a real contact email | README, CONTRIBUTING |
| updater `pubkey` | your own key (see section 2) | `tauri.conf.json` |

## 7. Upstream project content you should rewrite or remove
- `src/content/release-notes/` and `public/release-notes/`: the original project's changelog.
- `CONTRIBUTING.md`, `AGENTS.md`, `CLAUDE.md`, `.github/ISSUE_TEMPLATE/`, `.github/PULL_REQUEST_TEMPLATE.md`: written for the original project's community and policies (for example its feature freeze). Edit for how you want to work.
- Comments that say "upstream issue #123" refer to the original project's issue tracker.
- README sections that describe the original project's history, integrations and troubleshooting.

## 8. How to publish a release (installers on GitHub)
1. Put the project in the GitHub repository `mcgarrahjohn/quill` (public).
2. In the repo go to **Settings → Secrets and variables → Actions** and add:
   - `TAURI_SIGNING_PRIVATE_KEY`: the full contents of `~/.tauri/quill.key` on your Mac (never commit this file).
   - `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: leave empty (the key was created without a password).
3. Go to **Actions → Release → Run workflow**. GitHub builds macOS (Apple Silicon and Intel), Windows (x64, ARM) and Linux (x64, ARM) and attaches the installers to a **draft** release named `v<version>`.
4. Open the draft, check the files, and click **Publish release**.
5. On the landing page, set `RELEASE.repo`, `RELEASE.version` and `RELEASE.live = true`. The download button then points at the files listed below.

Files the page expects on the release (version `0.1.0`):
`Quill_0.1.0_aarch64.dmg`, `Quill_0.1.0_x64.dmg`, `Quill_0.1.0_x64-setup.exe`, `Quill_0.1.0_x64_en-US.msi`, `Quill_0.1.0_arm64-setup.exe`, `Quill_0.1.0_arm64_en-US.msi`, `Quill_0.1.0_amd64.AppImage`, `Quill_0.1.0_amd64.deb`, `Quill-0.1.0-1.x86_64.rpm`, `Quill_0.1.0_aarch64.AppImage`, `Quill_0.1.0_arm64.deb`, `Quill-0.1.0-1.aarch64.rpm`.
Installers are not signed yet, so macOS and Windows show a warning on first open. Do not commit installers into git; they belong on the release page.

## 9. Homebrew (`brew install --cask ...`)
`brew install --cask quill` only works if Homebrew's official list accepts Quill (needs a notable project and an Apple-signed, notarized app). Until then use your own tap:
1. Create a GitHub repo named `homebrew-quill`, with `Casks/quill.rb` copied from `packaging/homebrew/quill.rb`.
2. After each release, run `packaging/homebrew/update-cask.sh <version> mcgarrahjohn/quill` and push the updated file.
3. Users install with `brew install --cask mcgarrahjohn/quill/quill`. The landing page shows this command automatically once `RELEASE.live` is true and `RELEASE.repo` is set.
Note: while the app is unsigned, macOS shows its "unidentified developer" warning even for Homebrew installs.

## 10. Small things left from the original code
- Two English-only strings in `src/components/shared/ProgressBar.tsx` ("Downloading...") are not translated; a lint exception hides this. Add translation keys when convenient.
- The Windows signing step in the build config was removed; the workflow builds unsigned until you add signing accounts.

## 11. Speech models: decide before real users (DECISION: leave as is for now)
Quill currently downloads speech models from the original project's Hugging Face account (`handy-computer`) and its download server (`blob.handy.computer`). This works today and is left as is on purpose. Before telling real users:
- [ ] **Permission and reliability:** ask the original author whether other apps may use those files, or move to your own hosting. If they rename, remove or limit the files, Quill's model downloads break, and your users would blame Quill. A privacy-minded user watching network traffic will also see those connections.
- [ ] **Trim the list:** the catalog has 76 models; the app shows 29 by default (the "curated" ones). A short list of about 7 is a better product:
  - Parakeet 110M (tiny, English), Parakeet Unified English, Parakeet TDT v3 (25 languages)
  - Whisper Small, Whisper Large v3 Turbo (99 languages)
  - Qwen3-ASR 0.6B (30 languages), Canary 180M Flash
  - Keep the small voice-detection and language-detection helper models.
- [ ] **Licenses:** a few models are non-commercial (`cc-by-nc*`) or "other". Leave those out if you ever charge for Quill, and credit the `cc-by-4.0` ones.
- [ ] **If you host them yourself:** copy the chosen files to your own Hugging Face account or storage (`scripts/mirror_models.py` helps), then change the repository ids in `src-tauri/src/catalog/catalog.json` and the mirror address in `src-tauri/src/catalog/mod.rs`. The five older Whisper models are also public in the `ggerganov/whisper.cpp` Hugging Face repo.
Where the addresses live: `src-tauri/src/catalog/catalog.json`, `src-tauri/src/catalog/mod.rs`, `src-tauri/src/managers/model.rs`.

## 12. Fix filler words (on-device cleanup)
Built in: General → "Fix filler words and self-corrections", off by default. Off means no download and no extra memory. On downloads Qwen2.5-1.5B (about 1 GB) once, then cleans each dictation after the speech model, in about 0.2–0.5 s on a Mac (0.7 s CPU). A safety check pastes the original text if the cleaned text looks wrong.
- [ ] Test the Windows (x64 and ARM) and Linux builds from GitHub Actions: candle is untested there (CPU only).
- [ ] English-focused: other languages may be left unchanged or cleaned poorly. Consider saying so in the UI.
- [ ] Tricky corrections ("option A, actually no, option B") can still come out wrong; the safety check mostly catches this.
- [ ] Show the Qwen/candle credits on the About page (they are in NOTICE.md).
