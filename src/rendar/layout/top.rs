use crate::file::open_files;

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

            // クリアボタンを右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").clicked() {
                    files.clear();
                }

                if ui.button("Open").clicked() {
                    // ファイルダイアログを開くタイミングをずらす
                    *open_dialog = true;
                    ui.ctx().request_repaint();
                }
            });
        });
    });

    ui.separator();
}
