use crate::file::open_files;
use eframe::egui::DroppedFile;

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `files` - 開いているファイル
/// * `is_optimizing` - 最適化中かどうか
pub(crate) fn drop_files(
    dropped_files: &Vec<DroppedFile>,
    files: &mut open_files::OpenFiles,
    is_optimizing: &mut bool,
) {
    if dropped_files.is_empty() {
        return;
    }

    for file in dropped_files {
        if let Some(path) = &file.path {
            files.add_path(path.clone());
        }
    }

    *is_optimizing = true;
}
