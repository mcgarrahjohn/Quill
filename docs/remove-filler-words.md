# Remove filler words

An optional Quill feature that cleans up what you dictate. It is **off by default**. Find it in **Settings → General → Remove filler words**.

## What it does

- Removes fillers such as "um" and "uh".
- Applies self-corrections: "three, no wait, four" becomes "four".
- Changes nothing else. It does not reword, summarize or answer what you said.

Everything runs on your computer. Your voice and your text are never sent anywhere, and no other app (such as Ollama) is needed.

## How it works

1. **Your speech model** (for example Parakeet) turns your voice into text, exactly as before. The model you picked stays selected.
2. **A small cleanup model** (Qwen2.5-1.5B, about 1.1 GB) reads that text and rewrites it without the fillers. It runs on the Mac GPU on macOS and on the CPU on Windows and Linux.
3. **A safety check** compares the result to what you said. If the cleaned text added new content, answered a question instead of cleaning it, came out much longer, or dropped most of your words, Quill ignores it and pastes your original text. The worst case is an uncleaned sentence, never an invented one.

Very short phrases (under 4 words) are left alone. If cleanup takes longer than about 6 seconds, Quill pastes the original text.

## Speed and memory

| | Toggle off | Toggle on |
| --- | --- | --- |
| Download | none | about 1.1 GB, once |
| Extra memory | none | loaded while you dictate, freed after 10 idle minutes |
| Added time per dictation | none | about 0.2–0.5 s on a Mac, about 0.7 s on a CPU |

Turning the toggle off frees the memory immediately. The **Remove** button deletes the downloaded file.

## Limits

- It works best in English.
- Tricky corrections (for example "option A, actually no, option B…") can still come out wrong. The safety check catches most of these.

## Credits

Qwen2.5 (Alibaba Cloud / Qwen team, Apache-2.0) and candle (Hugging Face, MIT or Apache-2.0). See [NOTICE.md](../NOTICE.md).
