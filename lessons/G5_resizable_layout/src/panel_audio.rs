use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_audio_panel(&self, width: f32, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(width))
            .h_full()
            .bg(rgb(0x1d1d1d))
            .border_1()
            .border_color(rgb(0x2d2d2d))
            .flex()
            .flex_col()
            .p_1()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .justify_around()
                    .bg(rgb(0x0e0e0e))
                    .p_1()
                    .child(
                        // Left Audio track levels green-to-yellow gradient simulation
                        div()
                            .w_2()
                            .h_full()
                            .flex()
                            .flex_col()
                            .justify_end()
                            .child(div().h(gpui::relative(0.66)).w_full().bg(rgb(0x32cd32))) // Green levels
                    )
                    .child(
                        // Right Audio track levels
                        div()
                            .w_2()
                            .h_full()
                            .flex()
                            .flex_col()
                            .justify_end()
                            .child(div().h(gpui::relative(0.60)).w_full().bg(rgb(0x32cd32))) // Green levels
                    )
            )
            .child(
                div()
                    .w_full()
                    .flex()
                    .justify_center()
                    .text_xs()
                    .text_color(rgb(0x555555))
                    .child("dB")
            )
    }
}
