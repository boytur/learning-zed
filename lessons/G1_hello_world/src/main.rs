// G1 — GPUI Hello World
//
// CONCEPT: The absolute minimum GPUI app. This teaches the 4 core ideas:
//   1. Application  — the root of every GPUI program
//   2. App context (cx) — your handle to global state
//   3. open_window   — creates a window with a root "view"
//   4. Render trait  — how a view turns state into UI elements
//
// Real Zed source: crates/gpui/examples/hello_world.rs
// Docs: crates/gpui/README.md

use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};

// --- A "view" ---------------------------------------------------------------
// A view is an Entity that can be rendered. It holds your app's state.
// Here our state is just one string.

struct HelloWorld {
    text: SharedString,
}

// --- The Render trait -------------------------------------------------------
// GPUI calls `render` every frame. You build a tree of elements (divs, text,
// buttons...) and return it. GPUI lays it out and turns it into pixels.
//
// Note the signature: `&mut self` (your state), `&mut Window`, `&mut Context<Self>`.
// The `cx` is how you'd read/update other state or dispatch actions.

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // `div()` is GPUI's swiss-army-knife element. The `.flex()`, `.bg()`,
        // `.size()` etc. are a tailwind-style builder API.
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", self.text))
            .child(
                // A row of colored squares to show nesting + flex layout.
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(gpui::red()).rounded_md())
                    .child(div().size_8().bg(gpui::green()).rounded_md())
                    .child(div().size_8().bg(gpui::blue()).rounded_md()),
            )
    }
}

fn main() {
    // 1. Create the Application. `Application::new()` picks the windowing +
    //    text backend for the host OS (Win32 + DirectWrite on Windows).
    Application::new().run(|cx: &mut App| {
        // 2. Compute a centered 500x500 window bounds.
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        // 3. Open a window. The closure receives `cx` and returns the root view.
        //    `cx.new(...)` creates an Entity<HelloWorld>.
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| HelloWorld {
                    text: "World".into(),
                })
            },
        )
        .unwrap();

        // 4. Bring the window to the front.
        cx.activate(true);
    });
}

// --- Try it yourself --------------------------------------------------------
// 1. Change the text to your name.
// 2. Change the background color and window size.
// 3. Add a `.child()` with more text or a different color square.
// 4. (Harder) Add a second field to HelloWorld and render it.
