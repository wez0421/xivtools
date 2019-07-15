use imgui::{FontGlyphRange, ImFontConfig, ImGuiCol, ImStr, ImString, ImVec4};

// Combo boxes are annoying because I need slices of &ImStr, and I can't easily do that
// at compile time. This helper function takes a vector of ImStrings and handles the
// conversion to an appropriate slice.
pub fn combobox<'a>(
    ui: &imgui::Ui<'a>,
    label: &ImStr,
    items: &[ImString],
    mut pos: &mut i32,
) -> bool {
    let im_items: Vec<&ImStr> = items.iter().map(|l| l.as_ref()).collect();
    ui.combo(label, &mut pos, &im_items[..], im_items.len() as i32)
}

pub fn button<'a>(ui: &imgui::Ui<'a>, label: &str) -> bool {
    let im_label = ImString::new(label);
    ui.button(&im_label, (0.0, 0.0))
}

pub fn set_fonts(imgui: &mut imgui::ImGui, hidpi_factor: f64) {
    const FONT_SIZE: f64 = 16.0;
    let font_size = (FONT_SIZE * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("DroidSans.ttf"),
        ImFontConfig::new()
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size),
        &FontGlyphRange::default(),
    );
    imgui.set_font_global_scale((1.0 / hidpi_factor) as f32);
}

pub fn set_style(imgui: &mut imgui::ImGui) {
    let mut style = imgui.style_mut();
    // Set all windows / widgets to rectangles
    style.child_rounding = 0.0;
    style.popup_rounding = 0.0;
    style.frame_rounding = 0.0;
    style.window_rounding = 0.0;
    style.frame_border_size = 1.0;

    // This style is adapted from the light style in imgui_draw.cpp
    style.colors[ImGuiCol::Text as usize] = ImVec4 {
        x: 0.00,
        y: 0.00,
        z: 0.00,
        w: 1.00,
    };
    style.colors[ImGuiCol::TextDisabled as usize] = ImVec4 {
        x: 1.60,
        y: 1.60,
        z: 0.60,
        w: 1.00,
    };
    style.colors[ImGuiCol::WindowBg as usize] = ImVec4 {
        x: 0.94,
        y: 0.94,
        z: 0.94,
        w: 1.00,
    };
    style.colors[ImGuiCol::ChildBg as usize] = ImVec4 {
        x: 0.00,
        y: 0.00,
        z: 0.00,
        w: 0.00,
    };
    style.colors[ImGuiCol::PopupBg as usize] = ImVec4 {
        x: 1.00,
        y: 1.00,
        z: 1.00,
        w: 0.98,
    };
    style.colors[ImGuiCol::Border as usize] = ImVec4 {
        x: 0.00,
        y: 0.00,
        z: 0.00,
        w: 0.30,
    };
    style.colors[ImGuiCol::BorderShadow as usize] = ImVec4 {
        x: 0.00,
        y: 0.00,
        z: 0.00,
        w: 0.00,
    };
    style.colors[ImGuiCol::FrameBg as usize] = ImVec4 {
        x: 1.00,
        y: 1.00,
        z: 1.00,
        w: 1.00,
    };
    style.colors[ImGuiCol::FrameBgHovered as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.40,
    };
    style.colors[ImGuiCol::FrameBgActive as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.67,
    };
    style.colors[ImGuiCol::TitleBg as usize] = ImVec4 {
        x: 0.96,
        y: 0.96,
        z: 0.96,
        w: 1.00,
    };
    style.colors[ImGuiCol::TitleBgActive as usize] = ImVec4 {
        x: 0.82,
        y: 0.82,
        z: 0.82,
        w: 1.00,
    };
    style.colors[ImGuiCol::TitleBgCollapsed as usize] = ImVec4 {
        x: 1.00,
        y: 1.00,
        z: 1.00,
        w: 0.51,
    };
    style.colors[ImGuiCol::MenuBarBg as usize] = ImVec4 {
        x: 0.86,
        y: 0.86,
        z: 0.86,
        w: 1.00,
    };
    style.colors[ImGuiCol::ScrollbarBg as usize] = ImVec4 {
        x: 0.98,
        y: 0.98,
        z: 0.98,
        w: 0.53,
    };
    style.colors[ImGuiCol::ScrollbarGrab as usize] = ImVec4 {
        x: 0.69,
        y: 0.69,
        z: 0.69,
        w: 0.80,
    };
    style.colors[ImGuiCol::ScrollbarGrabHovered as usize] = ImVec4 {
        x: 0.49,
        y: 0.49,
        z: 0.49,
        w: 0.80,
    };
    style.colors[ImGuiCol::ScrollbarGrabActive as usize] = ImVec4 {
        x: 0.49,
        y: 0.49,
        z: 0.49,
        w: 1.00,
    };
    style.colors[ImGuiCol::CheckMark as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 1.00,
    };
    style.colors[ImGuiCol::SliderGrab as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.78,
    };
    style.colors[ImGuiCol::SliderGrabActive as usize] = ImVec4 {
        x: 0.46,
        y: 0.54,
        z: 0.80,
        w: 0.60,
    };
    style.colors[ImGuiCol::Button as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.40,
    };
    style.colors[ImGuiCol::ButtonHovered as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 1.00,
    };
    style.colors[ImGuiCol::ButtonActive as usize] = ImVec4 {
        x: 0.06,
        y: 0.53,
        z: 0.98,
        w: 1.00,
    };
    style.colors[ImGuiCol::Header as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.31,
    };
    style.colors[ImGuiCol::HeaderHovered as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.80,
    };
    style.colors[ImGuiCol::HeaderActive as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 1.00,
    };
    style.colors[ImGuiCol::Separator as usize] = ImVec4 {
        x: 0.39,
        y: 0.39,
        z: 0.39,
        w: 1.00,
    };
    style.colors[ImGuiCol::SeparatorHovered as usize] = ImVec4 {
        x: 0.14,
        y: 0.44,
        z: 0.80,
        w: 0.78,
    };
    style.colors[ImGuiCol::SeparatorActive as usize] = ImVec4 {
        x: 0.14,
        y: 0.44,
        z: 0.80,
        w: 1.00,
    };
    style.colors[ImGuiCol::ResizeGrip as usize] = ImVec4 {
        x: 0.80,
        y: 0.80,
        z: 0.80,
        w: 0.56,
    };
    style.colors[ImGuiCol::ResizeGripHovered as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.67,
    };
    style.colors[ImGuiCol::ResizeGripActive as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.95,
    };
    style.colors[ImGuiCol::PlotLines as usize] = ImVec4 {
        x: 0.39,
        y: 0.39,
        z: 0.39,
        w: 1.00,
    };
    style.colors[ImGuiCol::PlotLinesHovered as usize] = ImVec4 {
        x: 1.00,
        y: 0.43,
        z: 0.35,
        w: 1.00,
    };
    style.colors[ImGuiCol::PlotHistogram as usize] = ImVec4 {
        x: 0.90,
        y: 0.70,
        z: 0.00,
        w: 1.00,
    };
    style.colors[ImGuiCol::PlotHistogramHovered as usize] = ImVec4 {
        x: 1.00,
        y: 0.45,
        z: 0.00,
        w: 1.00,
    };
    style.colors[ImGuiCol::TextSelectedBg as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.35,
    };
    style.colors[ImGuiCol::DragDropTarget as usize] = ImVec4 {
        x: 0.26,
        y: 0.59,
        z: 0.98,
        w: 0.95,
    };
    style.colors[ImGuiCol::NavHighlight as usize] = style.colors[ImGuiCol::HeaderHovered as usize];
    style.colors[ImGuiCol::NavWindowingHighlight as usize] = ImVec4 {
        x: 0.70,
        y: 0.70,
        z: 0.70,
        w: 0.70,
    };
    style.colors[ImGuiCol::NavWindowingDimBg as usize] = ImVec4 {
        x: 0.20,
        y: 0.20,
        z: 0.20,
        w: 0.20,
    };
    style.colors[ImGuiCol::ModalWindowDimBg as usize] = ImVec4 {
        x: 0.20,
        y: 0.20,
        z: 0.20,
        w: 0.35,
    };
}
