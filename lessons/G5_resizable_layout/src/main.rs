// G5 — Resizable Premiere Pro Layout + UI
//
// CONCEPT: Resizable panel grids and rich desktop-style application UI.
// This lesson teaches:
//   1. Mouse event tracking (on_mouse_down, on_mouse_move, on_mouse_up)
//   2. How to handle drag-to-resize split panes reactively
//   3. Rich styling with dark-theme professional application controls
//   4. Code modularity: spreading components into clean, dedicated modules

use gpui::{
    App, Application, Bounds, Context, FocusHandle, Focusable, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, SharedString, Window, WindowBounds, WindowOptions, div, svg,
    prelude::*, px, rgb, size,
};

// Declare panel submodules
mod assets;
mod header;
mod panel_audio;
mod panel_program;
mod panel_project;
mod panel_properties;
mod panel_source;
mod panel_timeline;

use assets::LocalAssets;

// --- State & Layout Structs -------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum DraggedDivider {
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone)]
pub(crate) struct DirectoryItem {
    pub(crate) name: String,
    pub(crate) is_dir: bool,
    pub(crate) path: std::path::PathBuf,
}

pub(crate) struct PremiereLayout {
    pub(crate) focus_handle: FocusHandle,
    
    // Layout ratios (representing relative sizes of panels)
    pub(crate) top_height_ratio: f32,
    pub(crate) top_left_ratio: f32,
    pub(crate) top_middle_ratio: f32,
    pub(crate) bottom_left_ratio: f32,
    pub(crate) bottom_middle_ratio: f32,
    
    // Resizing drag state
    pub(crate) dragged_divider: Option<DraggedDivider>,
    
    // UI state
    pub(crate) selected_tool: SharedString,
    pub(crate) selected_top_left_tab: SharedString,
    pub(crate) selected_top_middle_tab: SharedString,
    pub(crate) selected_bottom_left_tab: SharedString,
    pub(crate) status_message: SharedString,
    
    // Filesystem state
    pub(crate) local_drives: Vec<String>,
    pub(crate) current_directory: std::path::PathBuf,
    pub(crate) directory_items: Vec<DirectoryItem>,
}

// --- Event Handlers ---------------------------------------------------------

impl PremiereLayout {
    fn handle_mouse_down(
        &mut self,
        divider: DraggedDivider,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.dragged_divider = Some(divider);
        self.status_message = format!("Dragging: {:?}", divider).into();
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(divider) = self.dragged_divider {
            let size = window.viewport_size();
            let win_w: f32 = size.width.into();
            let win_h: f32 = size.height.into();
            
            // Layout vertical starts after the header (around 80px)
            let header_offset = 80.0;
            let content_h = (win_h - header_offset).max(100.0);
            
            let mouse_x: f32 = event.position.x.into();
            let mouse_y: f32 = event.position.y.into();

            match divider {
                DraggedDivider::Horizontal => {
                    let relative_y = mouse_y - header_offset;
                    self.top_height_ratio = (relative_y / content_h).clamp(0.15, 0.85);
                }
                DraggedDivider::TopLeft => {
                    let new_left = (mouse_x / win_w).clamp(0.1, 0.8);
                    // Ensure the middle pane remains at least 10%
                    if new_left + self.top_middle_ratio < 0.9 {
                        self.top_left_ratio = new_left;
                    }
                }
                DraggedDivider::TopRight => {
                    let new_right_divider = (mouse_x / win_w).clamp(0.2, 0.9);
                    if new_right_divider > self.top_left_ratio + 0.1 {
                        self.top_middle_ratio = new_right_divider - self.top_left_ratio;
                    }
                }
                DraggedDivider::BottomLeft => {
                    let new_left = (mouse_x / win_w).clamp(0.1, 0.8);
                    if new_left + self.bottom_middle_ratio < 0.95 {
                        self.bottom_left_ratio = new_left;
                    }
                }
                DraggedDivider::BottomRight => {
                    let new_right_divider = (mouse_x / win_w).clamp(0.2, 0.95);
                    if new_right_divider > self.bottom_left_ratio + 0.1 {
                        self.bottom_middle_ratio = new_right_divider - self.bottom_left_ratio;
                    }
                }
            }
            cx.notify();
        }
    }

    fn handle_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.dragged_divider.is_some() {
            self.dragged_divider = None;
            self.status_message = "Layout ready".into();
            cx.notify();
        }
    }

    pub(crate) fn load_directory(
        &mut self,
        path: std::path::PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.current_directory = path.clone();
        self.directory_items.clear();
        
        // Add parent directory link if possible
        if let Some(parent) = self.current_directory.parent() {
            self.directory_items.push(DirectoryItem {
                name: "..".to_string(),
                is_dir: true,
                path: parent.to_path_buf(),
            });
        }
        
        self.status_message = "Scanning directory...".into();
        cx.notify();
        
        // Spawn async task to scan directory in the background
        cx.spawn_in(window, async move |this, cx| {
            let items = cx.background_executor().spawn(async move {
                let mut result_items = Vec::new();
                if let Ok(entries) = std::fs::read_dir(&path) {
                    for entry in entries.filter_map(Result::ok) {
                        let path = entry.path();
                        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                        let is_dir = path.is_dir();
                        if !name.starts_with('.') {
                            result_items.push(DirectoryItem { name, is_dir, path });
                        }
                    }
                    result_items.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
                }
                result_items
            }).await;
            
            this.update_in(cx, |this, _, cx| {
                this.directory_items.extend(items);
                this.status_message = "Layout ready".into();
                cx.notify();
            }).ok();
        }).detach();
    }
}

// --- Shared Helper Components -----------------------------------------------

impl PremiereLayout {
    // A tab item that highlights if selected
    pub(crate) fn render_tab(
        &self,
        name: &str,
        current: &SharedString,
        tab_state: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let name_owned = name.to_string();
        let name_clone = name_owned.clone();
        let is_active = current.as_ref() == name;
        
        div()
            .px_3()
            .py_1()
            .text_sm()
            .font_weight(if is_active { gpui::FontWeight::BOLD } else { gpui::FontWeight::NORMAL })
            .text_color(if is_active { rgb(0xffffff) } else { rgb(0x8c8c8c) })
            .border_b(if is_active { px(2.) } else { px(0.) })
            .border_color(rgb(0x1473e6)) // active blue
            .hover(|style| style.text_color(rgb(0xffffff)).cursor_pointer())
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                match tab_state {
                    "top_left" => this.selected_top_left_tab = name_clone.clone().into(),
                    "top_middle" => this.selected_top_middle_tab = name_clone.clone().into(),
                    "bottom_left" => this.selected_bottom_left_tab = name_clone.clone().into(),
                    _ => {}
                }
                cx.notify();
            }))
            .child(name_owned)
    }

    // A tool button in the vertical toolbar
    pub(crate) fn render_tool_button(
        &self,
        name: &str,
        icon: &'static str,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let name_owned = name.to_string();
        let is_active = self.selected_tool.as_ref() == name;
        
        div()
            .size_8()
            .flex()
            .justify_center()
            .items_center()
            .rounded_md()
            .bg(if is_active { rgb(0x1473e6) } else { rgb(0x282828) })
            .text_color(if is_active { rgb(0xffffff) } else { rgb(0xaaaaaa) })
            .hover(|style| style.bg(if is_active { rgb(0x1473e6) } else { rgb(0x383838) }).cursor_pointer())
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                this.selected_tool = name_owned.clone().into();
                this.status_message = format!("Selected Tool: {}", name_owned).into();
                cx.notify();
            }))
            .child(
                svg()
                    .path(icon)
                    .size(px(16.))
                    .text_color(if is_active { rgb(0xffffff) } else { rgb(0xaaaaaa) })
            )
    }
}

// --- Render Main Layout Shell -----------------------------------------------

impl Render for PremiereLayout {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let size = window.viewport_size();
        let total_w: f32 = size.width.into();
        let total_h: f32 = size.height.into();
        
        // Header & Menu bar: height is 80px fixed
        let header_h = 80.0;
        let main_h = (total_h - header_h).max(200.0);
        
        // Compute horizontal splits
        let top_height = (self.top_height_ratio * main_h).clamp(100.0, main_h - 100.0);
        let bottom_height = main_h - top_height - 4.0; // Subtract horizontal divider (4px)
        
        // Compute vertical splits for Top Panels
        let top_left_w = (self.top_left_ratio * total_w).clamp(100.0, total_w - 200.0);
        let top_middle_w = (self.top_middle_ratio * total_w).clamp(100.0, total_w - top_left_w - 100.0);
        let top_right_w = (total_w - top_left_w - top_middle_w - 8.0).max(50.0); // 2 dividers = 8px
        
        // Compute vertical splits for Bottom Panels
        let bottom_left_w = (self.bottom_left_ratio * total_w).clamp(100.0, total_w - 200.0);
        let bottom_middle_w = (self.bottom_middle_ratio * total_w).clamp(100.0, total_w - bottom_left_w - 100.0);
        let bottom_right_w = (total_w - bottom_left_w - bottom_middle_w - 8.0).max(40.0); // 2 dividers = 8px

        // Global mouse movement & mouse up tracking on the outermost div allows smooth dragging anywhere
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1a1a1a))
            .text_color(rgb(0xdddddd))
            .track_focus(&self.focus_handle(cx))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            
            // --- HEADER & NAVIGATION ---
            .child(self.render_header(cx))
            
            // --- MAIN PANELS AREA ---
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h(px(main_h))
                    
                    // --- TOP PANELS ROW ---
                    .child(
                        div()
                            .flex()
                            .w_full()
                            .h(px(top_height))
                            
                            // Top-Left Panel (Source Panel)
                            .child(self.render_source_panel(top_left_w, cx))
                            
                            // Top-Left Divider
                            .child(
                                div()
                                    .w(px(4.))
                                    .h_full()
                                    .bg(if self.dragged_divider == Some(DraggedDivider::TopLeft) { rgb(0x1473e6) } else { rgb(0x161616) })
                                    .hover(|style| style.bg(rgb(0x1473e6)))
                                    .cursor_ew_resize()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, e, window, cx| {
                                        this.handle_mouse_down(DraggedDivider::TopLeft, e, window, cx);
                                    }))
                            )
                            
                            // Top-Middle Panel (Program Monitor)
                            .child(self.render_program_panel(top_middle_w, cx))
                            
                            // Top-Right Divider
                            .child(
                                div()
                                    .w(px(4.))
                                    .h_full()
                                    .bg(if self.dragged_divider == Some(DraggedDivider::TopRight) { rgb(0x1473e6) } else { rgb(0x161616) })
                                    .hover(|style| style.bg(rgb(0x1473e6)))
                                    .cursor_ew_resize()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, e, window, cx| {
                                        this.handle_mouse_down(DraggedDivider::TopRight, e, window, cx);
                                    }))
                            )
                            
                            // Top-Right Panel (Properties)
                            .child(self.render_properties_panel(top_right_w, cx))
                    )
                    
                    // --- HORIZONTAL SPLIT DIVIDER ---
                    .child(
                        div()
                            .h(px(4.))
                            .w_full()
                            .bg(if self.dragged_divider == Some(DraggedDivider::Horizontal) { rgb(0x1473e6) } else { rgb(0x161616) })
                            .hover(|style| style.bg(rgb(0x1473e6)))
                            .cursor_ns_resize()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, e, window, cx| {
                                this.handle_mouse_down(DraggedDivider::Horizontal, e, window, cx);
                            }))
                    )
                    
                    // --- BOTTOM PANELS ROW ---
                    .child(
                        div()
                            .flex()
                            .w_full()
                            .h(px(bottom_height))
                            
                            // Bottom-Left Panel (Project Files / Media Browser)
                            .child(self.render_project_panel(bottom_left_w, bottom_height, cx))
                            
                            // Bottom-Left Divider
                            .child(
                                div()
                                    .w(px(4.))
                                    .h_full()
                                    .bg(if self.dragged_divider == Some(DraggedDivider::BottomLeft) { rgb(0x1473e6) } else { rgb(0x161616) })
                                    .hover(|style| style.bg(rgb(0x1473e6)))
                                    .cursor_ew_resize()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, e, window, cx| {
                                        this.handle_mouse_down(DraggedDivider::BottomLeft, e, window, cx);
                                    }))
                            )
                            
                            // Bottom-Middle Panel (Timeline Area)
                            .child(self.render_timeline_panel(bottom_middle_w, cx))
                            
                            // Bottom-Right Divider
                            .child(
                                div()
                                    .w(px(4.))
                                    .h_full()
                                    .bg(if self.dragged_divider == Some(DraggedDivider::BottomRight) { rgb(0x1473e6) } else { rgb(0x161616) })
                                    .hover(|style| style.bg(rgb(0x1473e6)))
                                    .cursor_ew_resize()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, e, window, cx| {
                                        this.handle_mouse_down(DraggedDivider::BottomRight, e, window, cx);
                                    }))
                            )
                            
                            // Bottom-Right Panel (Audio Levels Meter)
                            .child(self.render_audio_panel(bottom_right_w, cx))
                    )
            )
    }
}

impl Focusable for PremiereLayout {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn main() {
    Application::new().with_assets(LocalAssets).run(|cx: &mut App| {
        // Centered window, standard HD aspect ratio for video workspace
        let bounds = Bounds::centered(None, size(px(1100.), px(700.)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|cx| {
                    let mut layout = PremiereLayout {
                        focus_handle: cx.focus_handle(),
                        top_height_ratio: 0.55,
                        top_left_ratio: 0.35,
                        top_middle_ratio: 0.45,
                        bottom_left_ratio: 0.25,
                        bottom_middle_ratio: 0.65,
                        dragged_divider: None,
                        selected_tool: "Select".into(),
                        selected_top_left_tab: "Source: (no clips)".into(),
                        selected_top_middle_tab: "Program: (no sequences)".into(),
                        selected_bottom_left_tab: "Project: Untitled".into(),
                        status_message: "Layout ready".into(),
                        local_drives: Vec::new(),
                        current_directory: std::path::PathBuf::new(),
                        directory_items: Vec::new(),
                    };
                    
                    // Detect drives based on OS
                    #[cfg(target_os = "windows")]
                    {
                        for letter in b'A'..=b'Z' {
                            let drive = format!("{}:\\", letter as char);
                            if std::path::Path::new(&drive).exists() {
                                layout.local_drives.push(format!("{}:", letter as char));
                            }
                        }
                    }
                    
                    #[cfg(not(target_os = "windows"))]
                    {
                        // On macOS/Linux, the root path is "/"
                        layout.local_drives.push("/".to_string());
                        
                        // Check for mounted volumes on macOS under /Volumes
                        if let Ok(entries) = std::fs::read_dir("/Volumes") {
                            for entry in entries.filter_map(Result::ok) {
                                if let Some(name) = entry.file_name().to_str() {
                                    layout.local_drives.push(format!("/Volumes/{}", name));
                                }
                            }
                        }
                    }
                    
                    layout
                });
                
                view.update(cx, |this, cx| {
                    if let Ok(dir) = std::env::current_dir() {
                        this.load_directory(dir, window, cx);
                    } else if !this.local_drives.is_empty() {
                        let first_drive = std::path::PathBuf::from(&this.local_drives[0]);
                        this.load_directory(first_drive, window, cx);
                    }
                });
                
                view
            },
        )
        .unwrap();
        
        cx.activate(true);
    });
}
