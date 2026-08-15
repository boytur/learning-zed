# Learning Zed — Lesson Index

Each folder is one lesson. Read them in order.

## G1 — Hello World (make a window)
**Concept:** The absolute minimum GPUI app.

Everything starts with an `Application`. You create one with `Application::new()`,
run it with `.run(|cx| ...)`, open a window with `cx.open_window(...)`, and give it
a root view that implements the `Render` trait.

**Real source:** `crates/gpui/examples/hello_world.rs`

## G3 — Desktop file opening
**Concept:** The desktop file workflow — native dialogs + direct filesystem access.

This is the desktop version of `<input type="file">` + `fetch()`. The app opens a
native OS dialog (`prompt_for_paths`), reads the file from disk (`std::fs`), saves
to a new path (`prompt_for_new_path`), reveals it in Explorer (`reveal_path`), and
opens it with the system (`open_with_system`).

**Real source:** `crates/gpui/src/app.rs:1523`

## G5 — Resizable Premiere Pro Layout + UI
**Concept:** Custom resizable layouts, split-pane dragging, cursor customization, modular code structures, scroll-bound containers, and asynchronous filesystem loading.

We handle `on_mouse_down` on subtle divider borders to set drag-active states, track global movement with `on_mouse_move` and `on_mouse_up` on the root container, and update panel ratios reactively. 

This lesson also demonstrates how to:
- Modularize large Views into separate files using shared `impl View` blocks.
- Perform asynchronous directory scans on a background thread using GPUI's background executor to avoid blocking the main UI thread.
- Enable vertical scrolling on layout boundaries by defining unique component IDs and overflow configurations.
- Detect OS-specific mounts (Windows drive letters vs Mac `/Volumes`).
