use std::path::PathBuf;

/// ダブルクリックで Finder でファイルを選択表示する
/// * `path` - ファイルのパス
pub fn double_click(path: &PathBuf) {
    if let Err(err) = reveal_file_command(path) {
        eprintln!("Error revealing file: {}", err);
    }
}

/// Finder でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn reveal_file_command(path: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("open")
        .args(["-R", path.to_str().unwrap()])
        .status()?;
    Ok(())
}

/// Explorer でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "windows")]
fn reveal_file_command(path: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("explorer")
        .args(["/select,", path.to_str().unwrap()])
        .status()?;
    Ok(())
}

/// XDG でファイルを選択表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "linux")]
fn reveal_file_command(path: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("xdg-open")
        .args(["--reveal", path.to_str().unwrap()])
        .status()?;
    Ok(())
}
