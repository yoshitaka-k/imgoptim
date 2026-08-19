mod main;
mod assets;
mod setting;

pub use main::view::Rendar;

// パネルの背景色
pub(crate) const DARK_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(35, 35, 35);
pub(crate) const LIGHT_MODE_PANEL_COLOR: egui::Color32 = egui::Color32::from_rgb(225, 225, 225);

/// 設定タブ
#[derive(PartialEq)]
pub enum SettingTab {
    Concurrent,
    Quality,
    About,
}

impl SettingTab {
    fn to_string(&self) -> &str {
        match self {
            SettingTab::Concurrent => "Concurrent",
            SettingTab::Quality => "Quality",
            SettingTab::About => "About",
        }
    }
}

/// パネルの背景色
/// * `ui` - UI
/// * `return` - パネルの背景色
pub(crate) fn panel_fill_color(ui: &egui::Ui) -> egui::Color32 {
    if ui.ctx().global_style().visuals.dark_mode {
        DARK_MODE_PANEL_COLOR
    } else {
        LIGHT_MODE_PANEL_COLOR
    }
}

/// パネルのスタイルを設定
/// * `ui` - UI
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui) -> egui::Frame {
    let panel_fill_color = panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 2,
            bottom: 3,
        })
}
