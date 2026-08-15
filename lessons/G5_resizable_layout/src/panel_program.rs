use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_program_panel(&self, width: f32, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(self.render_tab("Program: (no sequences)", &self.selected_top_middle_tab, "top_middle", cx))
                    .child(self.render_tab("Reference Monitor", &self.selected_top_middle_tab, "top_middle", cx))
            )
            .child(
                // Video preview screen black box
                div()
                    .flex_1()
                    .bg(rgb(0x0a0a0a))
                    .flex()
                    .flex_col()
                    .justify_between()
                    .p_3()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .child(div().text_xs().text_color(rgb(0x666666)).child("Fit (100%)"))
                            .child(div().text_xs().text_color(rgb(0x1473e6)).child("00:00:00:00"))
                    )
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .justify_center()
                            .text_sm()
                            .text_color(rgb(0x444444))
                            .child("Drop clip or create sequence to preview")
                    )
                    .child(
                        div()
                            .flex()
                            .justify_center()
                            .gap_4()
                            .text_xs()
                            .text_color(rgb(0x8c8c8c))
                            .child("◀")
                            .child("▶")
                            .child("◉ Mark In")
                            .child("◉ Mark Out")
                    )
            )
    }
}
