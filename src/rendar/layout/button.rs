use crate::app;
use crate::file::open_file;

/// 下部ボタンを表示
/// * `ui` - UI
/// * `app` - アプリケーション
/// * `file` - ドロップされたファイル
pub(crate) fn bottom_button(
    ui: &mut egui::Ui,
    app: &app::App,
    file: &mut open_file::OpenFile,
    is_optimizing: &mut bool) {
    // 左寄せ
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        if ui.button("Open").clicked() {
            let extensions: Vec<String> = app.extensions()
                .iter()
                .map(|ext| ext.to_string())
                .collect();

            // Macのみファイルとフォルダを同時選択できる
            #[cfg(target_os = "macos")]
            let paths = rfd::FileDialog::new()
                .add_filter("Images", &extensions)
                .pick_files_or_folders();

            // Mac以外は複数フォルダ選択のみ
            #[cfg(not(target_os = "macos"))]
            let paths = rfd::FileDialog::new()
                .add_filter("Images", &extensions)
                .pick_folders();

            // ファイルを追加
            if let Some(paths) = paths {
                for path in paths {
                    file.add_path(path);
                }

                *is_optimizing = true;
            }
        }
    });

    // 右寄せ
    ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Clear").clicked() {
                file.clear();
            }
        });
    });

}
