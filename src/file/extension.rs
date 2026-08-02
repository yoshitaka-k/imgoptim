use std::ffi::OsStr;

#[derive(Clone, PartialEq)]
pub enum Extension {
    Jpeg,
    Png,
    Gif,
    Other,
}

impl Extension {
    pub fn from_str(extension: &OsStr) -> Self {
        match extension.to_ascii_lowercase().to_string_lossy().as_ref() {
            "jpg" | "jpeg" => Self::Jpeg,
            "png" => Self::Png,
            "gif" => Self::Gif,
            _ => Self::Other,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Other => "other",
        }
    }
}
