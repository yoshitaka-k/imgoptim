use std::ffi::OsStr;

#[derive(Clone, PartialEq)]
pub enum Extension {
    Jpeg,
    Png,
    Gif,
    None,
}

impl Extension {
    /// 文字列から Extension を作成
    /// * `extension` - 文字列
    /// * `return` - Extension
    pub fn from_str(extension: &OsStr) -> Self {
        match extension.to_ascii_lowercase().to_string_lossy().as_ref() {
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "gif" => Self::Gif,
            _ => Self::None,
        }
    }

    /// Extension を文字列に変換
    /// * `return` - 文字列
    pub fn to_str(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::None => "",
        }
    }
}
