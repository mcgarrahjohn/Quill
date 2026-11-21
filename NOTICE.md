# Notice and attribution

Quill's own source code is released under the MIT License. The full text, including the original copyright line, is kept unchanged in [LICENSE](LICENSE). Please leave that file in place.

Some libraries are included in this repository as local copies. Each keeps its own license file with its original copyright notice, which the MIT and Apache-2.0 licenses require:

| Library | Location | License |
| --- | --- | --- |
| `quill-keys` | `src-tauri/crates/quill-keys` | MIT |
| `quill-recorder` | `src-tauri/crates/quill-recorder` | MIT |
| `quill-cpal` (a fork of `cpal`) | `src-tauri/crates/quill-cpal` | Apache-2.0 |
| `@quill/models` | `packages/quill-models` | MIT |

## Fonts

Bundled in `src/assets/fonts/`, both under the SIL Open Font License 1.1:
- **Fraunces** (titles) from Undercase Type.
- **Hanken Grotesk** (interface text) from Hanken Design Co.

## Text cleanup (optional "Remove filler words" feature)

- **Qwen2.5-1.5B-Instruct** (Alibaba Cloud / Qwen team), Apache-2.0. Downloaded on request from Qwen's official Hugging Face repositories; never bundled.
- **candle** (Hugging Face), MIT or Apache-2.0. Runs the model on the user's device.
- `src-tauri/src/cleanup/qwen2q.rs` is a local copy of `quantized_qwen2.rs` from candle-transformers 0.11.1 (MIT or Apache-2.0) with one change: the model struct derives `Clone`.

## Speech models

- **NVIDIA Parakeet** models are licensed CC-BY-4.0, which requires credit to NVIDIA. Keep this line, and show it in the About page, while Parakeet is offered.
