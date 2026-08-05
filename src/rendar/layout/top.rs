use egui::Color32;

use crate::file::open_files;
use crate::rendar::assets::svg;

/// 上部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
pub(crate) fn top_layout(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    open_dialog: &mut bool,
) {
    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.label("File Drag & Drop");

            // 開くボタンとクリアボタンを右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // クリアボタン
                let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(18.0).tint(Color32::WHITE);
                if ui.button(clear_button).clicked() {
                    files.clear();
                }

                // 開くボタン
                let open_button = egui::Image::new(svg::FOLDER_OPEN).max_height(18.0).tint(Color32::WHITE);
                if ui.button(open_button).clicked() {
                    // ファイルダイアログを開くタイミングをずらす
                    *open_dialog = true;
                    ui.ctx().request_repaint();
                }
            });
        });
    });

    ui.separator();
}
