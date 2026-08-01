use crate::file::open_files;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, files: &open_files::OpenFiles) {
    for (index, path) in files.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(metadata) => {
                let size = metadata.len() / 1024;
                ui.label(format!(
                    "{} ({} KB)",
                    path.path().file_name().unwrap().to_str().unwrap(),
                    size
                ));

                if index < files.paths().len() - 1 {
                    ui.separator();
                }
            }
            Err(_) => {}
        }
    }
}
