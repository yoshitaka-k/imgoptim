use crate::file::open_files;
use eframe::egui::DroppedFileHandle;

/// ドロップされたファイルを処理
/// * `dropped_files` - ドロップされたファイル
/// * `files` - 開いているファイル
pub(crate) fn drop_files(
    dropped_files: &[DroppedFileHandle],
    files: &mut open_files::OpenFiles,
) -> Result<(), Box<dyn std::error::Error>> {
    if dropped_files.is_empty() {
        return Ok(());
    }

    for file in dropped_files {
        files.add_path(file.path().to_path_buf())?;
    }

    Ok(())
}
