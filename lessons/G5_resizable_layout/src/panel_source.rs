use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_source_panel(&self, width: f32, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(width))
            .h_full()
            .bg(rgb(0x1d1d1d))
            .border_1()
            .border_color(rgb(0x2d2d2d))
            .flex()
            .flex_col()
            .child(
                // Tabs
                div()
                    .flex()
                    .bg(rgb(0x181818))
                    .px_2()
                    .border_b(px(1.))
                    .border_color(rgb(0x282828))
                    .child(self.render_tab("Source: (no clips)", &self.selected_top_left_tab, "top_left", cx))
                    .child(self.render_tab("Effect Controls", &self.selected_top_left_tab, "top_left", cx))
                    .child(self.render_tab("Audio Clip Mixer", &self.selected_top_left_tab, "top_left", cx))
            )
            .child(
                // Content
                div()
                    .flex_1()
                    .bg(rgb(0x131313))
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .p_4()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x5a5a5a))
                            .child(format!("No media active in [{}]", self.selected_top_left_tab))
                    )
                    .child(
                        // Media Controls representation
                        div()
                            .flex()
                            .gap_4()
                            .mt_4()
                            .text_xs()
                            .text_color(rgb(0x777777))
                            .child("00:00:00:00")
                            .child("◀◀")
                            .child("◀")
                            .child("■")
                            .child("▶")
                            .child("▶▶")
                    )
            )
    }
}
