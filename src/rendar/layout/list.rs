use crate::file;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `file` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, file: &file::open_file::OpenFile) {
    for (index, path) in file.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(metadata) => {
                let size = metadata.len() / 1024;
                ui.label(format!(
                    "{} ({} KB)",
                    path.path().file_name().unwrap().to_str().unwrap(),
                    size
                ));

                if index < file.paths().len() - 1 {
                    ui.separator();
                }
            }
            Err(_) => {}
        }
    }
}
