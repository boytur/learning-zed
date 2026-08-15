use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_timeline_panel(&self, width: f32, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(width))
            .h_full()
            .bg(rgb(0x1d1d1d))
            .border_1()
            .border_color(rgb(0x2d2d2d))
            .flex()
            .child(
                // Left Toolbar of Timeline
                div()
                    .w_10()
                    .h_full()
                    .bg(rgb(0x222222))
                    .border_r(px(1.))
                    .border_color(rgb(0x2d2d2d))
                    .flex()
                    .flex_col()
                    .items_center()
                    .py_2()
                    .gap_2()
                    .child(self.render_tool_button("Select", "assets/icons/select.svg", cx))
                    .child(self.render_tool_button("Razor", "assets/icons/razor.svg", cx))
                    .child(self.render_tool_button("Slip", "assets/icons/slip.svg", cx))
                    .child(self.render_tool_button("Pen", "assets/icons/pen.svg", cx))
                    .child(self.render_tool_button("Hand", "assets/icons/hand.svg", cx))
                    .child(self.render_tool_button("Type", "assets/icons/type.svg", cx))
            )
            .child(
                // Main timeline tracks content
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .child(
                        // Timeline tabs header
                        div()
                            .flex()
                            .bg(rgb(0x181818))
                            .px_2()
                            .border_b(px(1.))
                            .border_color(rgb(0x282828))
                            .child(div().px_3().py_1().text_sm().text_color(rgb(0xffffff)).child("Timeline: (no sequences)"))
                    )
                    .child(
                        // Tracks display
                        div()
                            .flex_1()
                            .bg(rgb(0x141414))
                            .p_4()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .child(
                                // Time ruler
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_xs()
                                    .text_color(rgb(0x666666))
                                    .child("00:00:00:00")
                                    .child("00:05:00:00")
                                    .child("00:10:00:00")
                                    .child("00:15:00:00")
                            )
                            .child(
                                // Drag content area
                                div()
                                    .w_full()
                                    .flex()
                                    .justify_center()
                                    .text_xs()
                                    .text_color(rgb(0x444444))
                                    .child("Drop media here to create a sequence.")
                            )
                            .child(
                                // Track lines placeholder
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(div().h(px(4.)).w_full().bg(rgb(0x252525)))
                                    .child(div().h(px(4.)).w_full().bg(rgb(0x252525)))
                                    .child(div().h(px(4.)).w_full().bg(rgb(0x252525)))
                            )
                    )
            )
    }
}
