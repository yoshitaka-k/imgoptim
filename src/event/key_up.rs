use std::path::PathBuf;

/// スペースキーが押されたらファイルを選択表示する
/// * `path` - ファイルのパス
pub fn space_key(path: &PathBuf) {
    if let Err(err) = quicklook_command(path) {
        eprintln!("Error revealing file: {}", err);
    }
}

/// QuickLook でファイルを表示する
/// * `path` - ファイルのパス
/// * `return` - エラーが発生したかどうか
#[cfg(target_os = "macos")]
fn quicklook_command(path: &PathBuf) -> Result<(), std::io::Error> {
    std::process::Command::new("qlmanage")
        .args(["-p", path.to_str().unwrap()]).spawn()?;
    Ok(())
}
