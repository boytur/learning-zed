use crate::PremiereLayout;
use gpui::{div, prelude::*, px, rgb, svg, Context, MouseButton};

impl PremiereLayout {
    pub(crate) fn render_project_panel(
        &self,
        width: f32,
        height: f32,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content_h = (height - 36.0).max(100.0);

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
                    .child(self.render_tab(
                        "Project: Untitled",
                        &self.selected_bottom_left_tab,
                        "bottom_left",
                        cx,
                    ))
                    .child(self.render_tab(
                        "Media Browser",
                        &self.selected_bottom_left_tab,
                        "bottom_left",
                        cx,
                    ))
                    .child(self.render_tab(
                        "Effects",
                        &self.selected_bottom_left_tab,
                        "bottom_left",
                        cx,
                    )),
            )
            .child(
                // Files List / Explorer Layout
                div()
                    .h(px(content_h))
                    .flex()
                    .gap_2()
                    .p_2()
                    .child(
                        // Sidebar sidebar tree representation
                        div()
                            .id("project-sidebar")
                            .overflow_y_scroll()
                            .w_32()
                            .h(px(content_h))
                            .text_color(rgb(0x888888))
                            .text_xs()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(div().child("▼ Favorites").text_color(rgb(0xdddddd)))
                            .child(
                                div()
                                    .pl_3()
                                    .child("Learning Zed")
                                    .hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _, window, cx| {
                                            this.load_directory(
                                                std::path::PathBuf::from("D:\\workspaces\\learning-zed"),
                                                window,
                                                cx,
                                            );
                                        }),
                                    ),
                            )
                            .child(
                                div()
                                    .child("▼ Local Drives")
                                    .text_color(rgb(0xdddddd))
                                    .mt_2(),
                            )
                            .children(self.local_drives.iter().map(|drive| {
                                let drive_clone = drive.clone();
                                let full_path =
                                    std::path::PathBuf::from(format!("{}\\", drive_clone));
                                div()
                                    .pl_3()
                                    .child(format!("Drive {}", drive_clone))
                                    .hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer())
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, window, cx| {
                                            this.load_directory(full_path.clone(), window, cx);
                                        }),
                                    )
                            })),
                    )
                    .child(
                        // Main Files View
                        div()
                            .flex_1()
                            .h(px(content_h))
                            .flex()
                            .flex_col()
                            .child(
                                // Current Directory Path Breadcrumb
                                div()
                                    .px_2()
                                    .py_1()
                                    .bg(rgb(0x141414))
                                    .text_xs()
                                    .text_color(rgb(0x888888))
                                    .child(self.current_directory.to_string_lossy().to_string()),
                            )
                            .child(
                                // Files Grid
                                div()
                                    .id("files-grid")
                                    .overflow_y_scroll()
                                    .w_full()
                                    .h(px(content_h - 26.0))
                                    .flex()
                                    .flex_row()
                                    .flex_wrap()
                                    .gap_2()
                                    .p_2()
                                    .children(self.directory_items.iter().map(|item| {
                                        let item_path = item.path.clone();
                                        let is_dir = item.is_dir;
                                        let name = item.name.clone();
                                        let name_for_listener = name.clone();

                                        div()
                                            .size_24()
                                            .bg(rgb(0x252525))
                                            .border_1()
                                            .border_color(rgb(0x353535))
                                            .rounded_md()
                                            .flex()
                                            .flex_col()
                                            .justify_center()
                                            .items_center()
                                            .hover(|style| style.bg(rgb(0x303030)).cursor_pointer())
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _, window, cx| {
                                                    if is_dir {
                                                        this.load_directory(item_path.clone(), window, cx);
                                                    } else {
                                                        this.status_message = format!(
                                                            "Selected File: {}",
                                                            name_for_listener
                                                        )
                                                        .into();
                                                        cx.notify();
                                                    }
                                                }),
                                            )
                                            .child(
                                                svg()
                                                    .path(if is_dir {
                                                        "assets/icons/folder.svg"
                                                     } else {
                                                        "assets/icons/type.svg"
                                                     })
                                                    .size(px(28.))
                                                    .text_color(if is_dir {
                                                        rgb(0xd4af37)
                                                    } else {
                                                        rgb(0x888888)
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .mt_1()
                                                    .text_color(rgb(0xbbbbbb))
                                                    .child(if name.len() > 11 {
                                                        format!("{}...", &name[0..9])
                                                    } else {
                                                        name
                                                    }),
                                            )
                                    })),
                            ),
                    ),
            )
    }
}
