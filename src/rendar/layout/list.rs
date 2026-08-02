use crate::file::open_files;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, files: &open_files::OpenFiles) {
    for (index, path) in files.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(_) => {
                if *path.is_optimized() {
                    ui.label(format!(
                        "{} {} ({} KB -> {} KB) {}%",
                        "✅",
                        path.file_name(),
                        path.size() / 1024,
                        path.new_size() / 1024,
                        path.percent()
                    ));
                } else {
                    ui.label(format!(
                        "{} ({} KB)",
                        path.file_name(),
                        path.size() / 1024,
                    ));
                }

                if index < files.paths().len() - 1 {
                    ui.separator();
                }
            }
            Err(_) => {}
        }
    }
}
