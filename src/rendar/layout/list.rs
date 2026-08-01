use crate::drop_file;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `drop_file` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, drop_file: &drop_file::DropFile) {
    for (index, path) in drop_file.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(metadata) => {
                let size = metadata.len() / 1024;
                ui.label(format!(
                    "{} ({} KB)",
                    path.path().file_name().unwrap().to_str().unwrap(),
                    size
                ));

                if index < drop_file.paths().len() - 1 {
                    ui.separator();
                }
            }
            Err(_) => {}
        }
    }
}
