use gpui::{prelude::*, rgb, px, Context, div};
use crate::PremiereLayout;

impl PremiereLayout {
    pub(crate) fn render_header(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        let header_h = 80.0;
        
        div()
            .h(px(header_h))
            .w_full()
            .bg(rgb(0x222222))
            .border_b(px(1.))
            .border_color(rgb(0x111111))
            .flex()
            .flex_col()
            .justify_between()
            .child(
                // Top level application menu
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .px_4()
                    .py_1()
                    .bg(rgb(0x161616))
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .text_xs()
                            .text_color(rgb(0x888888))
                            .child(div().child("File").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Edit").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Sequence").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Markers").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Graphics").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("View").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Window").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Help").hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer())),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x555555))
                            .child("Adobe Premiere Pro Mockup (GPUI)")
                    )
            )
            .child(
                // Layout Tabs / Options
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .px_4()
                    .pb_1()
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .child(div().child("Import").text_color(rgb(0x8c8c8c)).hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer()))
                            .child(div().child("Edit").text_color(rgb(0xffffff)).border_b(px(2.)).border_color(rgb(0x1473e6)).font_weight(gpui::FontWeight::BOLD))
                            .child(div().child("Export").text_color(rgb(0x8c8c8c)).hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer())),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(rgb(0xdddddd))
                            .child("Untitled - Edited")
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .text_xs()
                            .text_color(rgb(0x8c8c8c))
                            .child(div().child("LEARNING").text_color(rgb(0x1473e6)))
                            .child(div().child(self.status_message.clone()))
                    )
            )
    }
}
