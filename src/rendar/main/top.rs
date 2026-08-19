use crate::file::open_files;
use crate::rendar::assets;
use crate::rendar::assets::{constants, svg};

/// 上部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
pub(crate) fn view(
    ui: &mut egui::Ui,
    _files: &mut open_files::OpenFiles,
    open_dialog: &mut bool,
    settings_window_open: &mut bool,
    settings_window_pos: &mut Option<egui::Pos2>,
) {
    // ボタンの色を設定
    let button_color = assets::button_icon_color(ui);

    ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::UPLOAD_FILE).max_height(constants::UPLOAD_FILE_ICON_SIZE).tint(button_color));
            ui.label("Folders or Files to Optimize Drag & Drop");
        });

        // 開くボタンとクリアボタンを右寄せ
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // 設定ボタン
            let settings_button = egui::Image::new(svg::SETTINGS).max_height(18.0).tint(button_color);
            if ui.button(settings_button).on_hover_text("Settings").clicked() {
                // 設定ダイアログを開く
                *settings_window_open = true;

                // 設定ダイアログの表示位置を設定
                *settings_window_pos = ui.ctx().input(|input| {
                    input.viewport().outer_rect.map(|rect| rect.min)
                });

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
}
