use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_properties_panel(&self, width: f32, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(width))
            .h_full()
            .bg(rgb(0x1d1d1d))
            .border_1()
            .border_color(rgb(0x2d2d2d))
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .bg(rgb(0x181818))
                    .px_2()
                    .border_b(px(1.))
                    .border_color(rgb(0x282828))
                    .child(div().px_3().py_1().text_sm().text_color(rgb(0xffffff)).child("Properties"))
            )
            .child(
                div()
                    .flex_1()
                    .p_3()
                    .text_xs()
                    .text_color(rgb(0x777777))
                    .child("No active sequence")
                    .child(
                        div()
                            .mt_4()
                            .child("Select a clip in the timeline to view properties details here.")
                    )
            )
    }
}
