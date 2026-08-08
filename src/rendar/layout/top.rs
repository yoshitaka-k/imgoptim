use crate::file::open_files;
use crate::rendar::assets::svg;
use crate::rendar::{DARK_MODE_BUTTON_COLOR, LIGHT_MODE_BUTTON_COLOR};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
pub(crate) fn top_layout(
    ui: &mut egui::Ui,
    _files: &mut open_files::OpenFiles,
    open_dialog: &mut bool,
    settings_window_open: &mut bool,
) {
    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.label("File Drag & Drop");

            // ボタンの色を設定
            let button_color = if ui.ctx().global_style().visuals.dark_mode {
                DARK_MODE_BUTTON_COLOR
            } else {
                LIGHT_MODE_BUTTON_COLOR
            };

            // 開くボタンとクリアボタンを右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // 設定ボタン
                let settings_button = egui::Image::new(svg::SETTINGS).max_height(18.0).tint(button_color);
                if ui.button(settings_button).on_hover_text("Settings").clicked() {
                    // 設定ダイアログを開く
                    *settings_window_open = true;
                    ui.ctx().request_repaint();
                }

                // 開くボタン
                let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(18.0).tint(button_color);
                if ui.button(open_button).on_hover_text("Files Open").clicked() {
                    // ファイルダイアログを開くタイミングをずらす
                    *open_dialog = true;
                    ui.ctx().request_repaint();
                }
            });
        });
    });

    ui.separator();
}
