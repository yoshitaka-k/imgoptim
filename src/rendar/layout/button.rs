use crate::file::open_files;

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `open_dialog` - ファイルダイアログを開くタイミングをずらす
pub(crate) fn bottom_button(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    open_dialog: &mut bool,
) {
    // 左寄せ
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open").clicked() {
                // ファイルダイアログを開くタイミングをずらす
                *open_dialog = true;
                ui.ctx().request_repaint();
            }

            // 右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Clear").clicked() {
                    files.clear();
                }
            });
        });
    });
}
