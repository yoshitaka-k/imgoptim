use serde::{Deserialize, Serialize};
use oxipng::Options;

/// PNG 圧縮タイプ
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngCompression {
    Min,
    Fast,
    Default,
    Best,
    Max,
}

impl PngCompression {
    /// PNG 圧縮タイプを oxipng の Options に変換
    /// * `return` - 圧縮タイプの Options
    pub fn to_options(&self) -> Options {
        match self {
            PngCompression::Min => Options::from_preset(0),
            PngCompression::Fast => Options::from_preset(1),
            PngCompression::Default => Options::from_preset(2),
            PngCompression::Best => Options::from_preset(4),
            PngCompression::Max => Options::from_preset(6),
        }
    }

    /// PNG 圧縮タイプを文字列に変換
    /// * `return` - 圧縮タイプの文字列
    pub fn to_string(&self) -> String {
        match self {
            PngCompression::Min => "Min".to_string(),
            PngCompression::Fast => "Fast".to_string(),
            PngCompression::Default => "Default".to_string(),
            PngCompression::Best => "Best".to_string(),
            PngCompression::Max => "Max".to_string(),
        }
    }
}
