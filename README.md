# Learning Zed — Desktop App Concepts

A hands-on Rust project for **full-stack web developers** who want to learn how
**desktop** apps work — the things that are different from the web.

Each lesson is a small, runnable GPUI app that demonstrates a desktop concept
(native dialogs, filesystem access, windows), with a README linking back to the
real Zed source.

## The web → desktop mental shift

| | Web | Desktop |
|---|---|---|
| File access | Browser sandbox; only files the user uploads | App reads/writes any file the user picks |
| File dialog | `<input type="file">` | Native OS dialog (`prompt_for_paths`) |
| Open a file | `fetch()` + upload | `std::fs::read_to_string()` |
| Reveal in file manager | *(no equivalent)* | `reveal_path()` → Explorer/Finder |
| Open with default app | *(no equivalent)* | `open_with_system()` |

## How to run a lesson

```bash
cargo run --example G1_hello_world
```

Replace `G1_hello_world` with any lesson name.

## Lessons

| # | Lesson | Concept | Real Zed source |
|---|--------|---------|-----------------|
| G1 | `G1_hello_world` | How to make a window (Application, App, Render) | `crates/gpui/examples/hello_world.rs` |
| G3 | `G3_file_open` | Desktop file workflow: native dialogs + filesystem | `crates/gpui/src/app.rs:1523` |

## Suggested reading order

1. **G1** — make a window (the foundation)
2. **G3** — open/save/reveal files (the desktop-specific part)

Each lesson's README has a "Try it yourself" section with exercises.
