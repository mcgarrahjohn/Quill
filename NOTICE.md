# Notice and attribution

Quill is derived from **Handy**, an open-source speech-to-text application.

- Original project: https://github.com/cjpais/Handy
- Original copyright: Copyright (c) 2025 CJ Pais
- License: MIT. The full text, including the original copyright line, is kept unchanged in [LICENSE](LICENSE).

Three libraries that Handy published separately are included in this repository as local copies under new names. Each keeps its own license file with the original copyright notice:

| Quill name | Location | Original name | License |
| --- | --- | --- | --- |
| `quill-keys` | `src-tauri/crates/quill-keys` | `handy-keys` 0.3.5 | MIT |
| `quill-recorder` | `src-tauri/crates/quill-recorder` | `handy-recorder` 0.1.1 | MIT |
| `quill-cpal` | `src-tauri/crates/quill-cpal` | `handy-cpal` 0.18.2 (a fork of `cpal`) | Apache-2.0 |
| `@quill/models` | `packages/quill-models` | `@handy-computer/models` 0.2.3 | MIT |

The MIT and Apache-2.0 licenses require these notices to stay with the code, which is why the word "Handy" still appears in those license files.

## Fonts

Bundled in `src/assets/fonts/`, both under the SIL Open Font License 1.1:
- **Fraunces** (titles) from Undercase Type.
- **Hanken Grotesk** (interface text) from Hanken Design Co.

## Text cleanup (optional "Fix filler words" feature)

- **Qwen2.5-1.5B-Instruct** (Alibaba Cloud / Qwen team), Apache-2.0. Downloaded on request from Qwen's official Hugging Face repositories; never bundled.
- **candle** (Hugging Face), MIT or Apache-2.0. Runs the model on the user's device.
- `src-tauri/src/cleanup/qwen2q.rs` is a local copy of `quantized_qwen2.rs` from candle-transformers 0.11.1 (MIT or Apache-2.0) with one change: the model struct derives `Clone`.

## Speech models

- **NVIDIA Parakeet** models are licensed CC-BY-4.0, which requires credit to NVIDIA. Keep this line, and show it in the About page, while Parakeet is offered.
