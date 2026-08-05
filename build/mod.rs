pub(crate) mod fonts;

/// フォントのパスを取得
/// * `path` - フォントのパス
/// * `return` - フォントのパス
pub(crate) fn include_assets_path(path: &str) -> String {
    if path.starts_with("assets/") {
        format!("concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\")", path)
    } else {
        panic!("assets path must start with 'assets/' (was: {})", path);
    }
}

/// 識別子として使える名前に変換
/// 空白や記号を『 _ 』に変換して大文字にする
/// * `font_name` - フォント名
/// * `return` - 識別子として使える名前
pub(crate) fn to_const_name(font_name: &str) -> String {
    font_name.chars()
              .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
              .collect::<String>()
              .to_uppercase()
}
