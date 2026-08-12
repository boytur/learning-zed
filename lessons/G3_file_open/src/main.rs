// G3 — Desktop file opening (the stuff web devs don't know)
//
// As a full-stack web dev, you know `<input type="file">` + `fetch()`.
// On the desktop it's different — the app talks DIRECTLY to the OS:
//
//   Web:    <input type="file">  →  browser sandbox  →  upload
//   Desktop: prompt_for_paths()  →  native OS dialog →  read from disk
//
// This app is a tiny file viewer that demonstrates the desktop file workflow:
//   1. Open a native file dialog (like Ctrl+O in Zed)
//   2. Read the selected file from disk
//   3. Display its contents
//   4. Save to a new path (native save dialog)
//   5. Reveal in Explorer / open with the system
//
// Real Zed source: crates/gpui/src/app.rs:1523 (prompt_for_paths)

use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, MouseButton, MouseUpEvent,
    PathPromptOptions, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb,
    size,
};
use std::path::PathBuf;

// --- State: what the app remembers ------------------------------------------

struct FileViewer {
    focus_handle: FocusHandle,
    opened_path: Option<PathBuf>,
    content: SharedString,
    status: SharedString,
}

// --- Handlers: the desktop file operations ----------------------------------

impl FileViewer {
    // 1. Open a native file dialog and read the selected file.
    //    This is the desktop version of `<input type="file">`.
    fn open_file(&mut self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        // Ask the OS to show its native "open" dialog.
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: None,
        });

        // The dialog is async — await the result on a spawned task.
        // Note: the receiver yields Result<Result<Option<Vec<PathBuf>>, Error>, Canceled>.
        cx.spawn_in(window, async move |this, cx| {
            let inner = match receiver.await {
                Ok(inner) => inner,
                Err(_) => return, // dialog cancelled
            };
            match inner {
                Ok(Some(paths)) => {
                    if let Some(path) = paths.first() {
                        // Read the file from disk (std::fs — direct OS access).
                        let content = std::fs::read_to_string(path).unwrap_or_else(|e| {
                            format!("Error reading file: {e}")
                        });
                        this.update_in(cx, |this, _, cx| {
                            this.opened_path = Some(path.clone());
                            this.content = content.into();
                            this.status = format!("Opened: {}", path.display()).into();
                            cx.notify();
                        })
                        .ok();
                    }
                }
                Ok(None) => {
                    this.update_in(cx, |this, _, cx| {
                        this.status = "Cancelled".into();
                        cx.notify();
                    })
                    .ok();
                }
                Err(e) => {
                    this.update_in(cx, |this, _, cx| {
                        this.status = format!("Dialog error: {e}").into();
                        cx.notify();
                    })
                    .ok();
                }
            }
        })
        .detach();
    }

    // 2. Save the current content to a new path (native save dialog).
    fn save_file(&mut self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.opened_path.clone() else {
            self.status = "Open a file first".into();
            cx.notify();
            return;
        };
        let dir = path.parent().unwrap_or(std::path::Path::new(".")).to_path_buf();
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned());
        let receiver = cx.prompt_for_new_path(&dir, name.as_deref());
        let content = self.content.clone();

        cx.spawn_in(window, async move |this, cx| {
            let inner = match receiver.await {
                Ok(inner) => inner,
                Err(_) => return,
            };
            if let Ok(Some(save_path)) = inner {
                // Write to disk (std::fs — direct OS access).
                let result = std::fs::write(&save_path, content.to_string());
                this.update_in(cx, |this, _, cx| {
                    this.status = match result {
                        Ok(()) => format!("Saved: {}", save_path.display()).into(),
                        Err(e) => format!("Save error: {e}").into(),
                    };
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }

    // 3. Reveal the file in the OS file manager (Explorer / Finder).
    fn reveal(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = self.opened_path.clone() {
            cx.reveal_path(&path); // opens Explorer/Finder at the file
        }
    }

    // 4. Open the file with the system's default application.
    fn open_with_system(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(path) = self.opened_path.clone() {
            cx.open_with_system(&path); // like double-clicking the file
        }
    }
}

// --- Render: the UI ----------------------------------------------------------

impl Render for FileViewer {
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
                    .child("Desktop File Viewer")
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x888888))
                            .child("The desktop version of <input type=file> + fetch()"),
                    ),
            )
            // The action buttons (inlined so cx.listener types match).
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        div()
                            .border_1()
                            .border_color(rgb(0x555555))
                            .rounded_md()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x333333))
                            .hover(|style| style.bg(rgb(0x444444)).cursor_pointer())
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::open_file))
                            .child("Open File..."),
                    )
                    .child(
                        div()
                            .border_1()
                            .border_color(rgb(0x555555))
                            .rounded_md()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x333333))
                            .hover(|style| style.bg(rgb(0x444444)).cursor_pointer())
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::save_file))
                            .child("Save As..."),
                    )
                    .child(
                        div()
                            .border_1()
                            .border_color(rgb(0x555555))
                            .rounded_md()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x333333))
                            .hover(|style| style.bg(rgb(0x444444)).cursor_pointer())
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::reveal))
                            .child("Reveal in Explorer"),
                    )
                    .child(
                        div()
                            .border_1()
                            .border_color(rgb(0x555555))
                            .rounded_md()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x333333))
                            .hover(|style| style.bg(rgb(0x444444)).cursor_pointer())
                            .on_mouse_up(MouseButton::Left, cx.listener(Self::open_with_system))
                            .child("Open with System"),
                    ),
            )
            // Status line.
            .child(div().text_sm().text_color(rgb(0x88ff88)).child(self.status.clone()))
            // The opened path.
            .child(div().text_sm().text_color(rgb(0x888888)).child(path))
            // The file contents (a simple text area).
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

impl Focusable for FileViewer {
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
                cx.new(|cx| FileViewer {
                    focus_handle: cx.focus_handle(),
                    opened_path: None,
                    content: "Click 'Open File...' to pick a file from disk.\n\n\
                              This is the desktop equivalent of a web file input — \
                              but here the app reads the file directly from the OS \
                              filesystem, no upload needed.".into(),
                    status: "Ready".into(),
                })
            },
        )
        .unwrap();
        cx.activate(true);
    });
}

// --- Try it yourself --------------------------------------------------------
// 1. Open a file and edit the content, then Save As to a new file.
// 2. Add a "New File" button that clears the content.
// 3. (Harder) Use `prompt_for_paths` with `multiple: true` to open several files.
