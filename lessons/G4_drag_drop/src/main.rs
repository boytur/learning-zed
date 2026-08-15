// G4 — Drag & drop files from the OS
//
// Drop a file from Explorer/Finder into the window and it opens.
// This is a desktop capability with NO real web equivalent — browsers
// block OS file drops for security.
//
// How it works in GPUI:
//   1. The OS tells GPUI "files are being dragged over the window"
//      → GPUI turns that into an internal drag with `ExternalPaths` as the value
//   2. A drop-target element registers `.on_drop(...)`
//   3. When the user releases, the handler receives `&ExternalPaths`
//   4. `paths.paths()` gives you the Vec<PathBuf> of dropped files
//
// Real Zed source:
//   - FileDropEvent:  crates/gpui/src/interactive.rs:728
//   - OS→internal:    crates/gpui/src/window.rs:5163
//   - open dropped:   crates/workspace/src/pane.rs:4046

use gpui::{
    App, Application, Bounds, Context, ExternalPaths, FocusHandle, Focusable, SharedString, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use std::path::PathBuf;

// --- State ------------------------------------------------------------------

struct DropViewer {
    focus_handle: FocusHandle,
    opened_path: Option<PathBuf>,
    content: SharedString,
    status: SharedString,
}

// --- The drop handler -------------------------------------------------------
// This is the desktop version of "user dropped a file on me".
// In the web you'd use a drag-and-drop zone + FileReader; here the OS hands
// us real filesystem paths and we read them directly.

impl DropViewer {
    fn handle_drop(&mut self, paths: &ExternalPaths, _window: &mut Window, cx: &mut Context<Self>) {
        // paths.paths() -> &[PathBuf] of everything the user dropped.
        let dropped = paths.paths();

        if dropped.is_empty() {
            self.status = "No files dropped".into();
            cx.notify();
            return;
        }

        // Open the first dropped file (like Zed opens the first of several).
        let path = dropped[0].clone();
        let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            format!("Error reading file: {e}")
        });

        self.opened_path = Some(path.clone());
        self.content = content.into();
        self.status = format!(
            "Dropped {} file(s) — opened: {}",
            dropped.len(),
            path.display()
        )
        .into();
        cx.notify();
    }
}

// --- Render ------------------------------------------------------------------

impl Render for DropViewer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let path = self
            .opened_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "(no file open)".to_string());

        div()
            .flex()
            .flex_col()
            .gap_3()
            .p_4()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .track_focus(&self.focus_handle(cx))
            .child(
                div()
                    .text_xl()
                    .child("Drag & Drop File Viewer")
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("Drop a file from Explorer/Finder into the box below"),
                    ),
            )
            // The drop target. `.on_drop` receives the dropped ExternalPaths.
            .child(
                div()
                    .id("drop-target")
                    .h_40()
                    .w_full()
                    .flex()
                    .justify_center()
                    .items_center()
                    .border_3()
                    .border_dashed()
                    .border_color(rgb(0x888888))
                    .rounded_md()
                    .text_color(rgb(0xaaaaaa))
                    .on_drop(cx.listener(Self::handle_drop))
                    .child("Drop files here"),
            )
            // Status line.
            .child(div().text_sm().text_color(rgb(0x88ff88)).child(self.status.clone()))
            // The opened path.
            .child(div().text_sm().text_color(rgb(0x888888)).child(path))
            // The file contents.
            .child(
                div()
                    .flex_1()
                    .border_1()
                    .border_color(rgb(0x444444))
                    .rounded_md()
                    .p_3()
                    .font_family("monospace")
                    .text_sm()
                    .child(self.content.clone()),
            )
    }
}

impl Focusable for DropViewer {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(700.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| DropViewer {
                    focus_handle: cx.focus_handle(),
                    opened_path: None,
                    content: "Drop a file into the dashed box above.\n\n\
                              The OS hands the app the file's real path, and the app \
                              reads it straight from disk — no upload, no sandbox.".into(),
                    status: "Ready — waiting for a drop".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

// --- Try it yourself --------------------------------------------------------
// 1. Drop a folder instead of a file — see what happens (it'll try to read it
//    as text). Use std::fs::metadata to detect directories.
// 2. Drop several files at once and open them all (loop over paths.paths()).
// 3. (Harder) Show a highlight on the drop target while files are hovering,
//    using the FileDropEvent::Entered / Exited states.
