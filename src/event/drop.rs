use crate::file::open_files;
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `files` - 開いているファイル
/// * `is_optimizing` - 最適化中かどうか
pub(crate) fn drop_files(
    dropped_files: &[DroppedFileHandle],
    files: &mut open_files::OpenFiles,
    is_optimizing: &mut bool,
) {
    if dropped_files.is_empty() {
        return;
    }

    for file in dropped_files {
        files.add_path(file.path().to_path_buf());
    }

    *is_optimizing = true;
}
