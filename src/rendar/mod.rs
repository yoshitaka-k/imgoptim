mod main;
mod assets;
mod setting;

pub use main::view::Rendar;

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

/// パネルのスタイルを設定
/// * `ui` - UI
/// * `return` - パネルのスタイル
pub(crate) fn panel_style(ui: &mut egui::Ui) -> egui::Frame {
    let panel_fill_color = assets::panel_fill_color(ui);
    egui::Frame::default()
        .fill(panel_fill_color)
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 2,
            bottom: 3,
        })
}
