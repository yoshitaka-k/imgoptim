use serde::{Deserialize, Serialize};
use oxipng::Options;

/// PNG プリセット
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngPreset {
    Min,
    Fast,
    Default,
    Best,
    Max,
}

impl PngPreset {
    /// PNG プリセットを oxipng の Options に変換
    /// * `return` - プリセットの Options
    pub fn to_options(&self) -> Options {
        match self {
            PngPreset::Min => Options::from_preset(0),
            PngPreset::Fast => Options::from_preset(1),
            PngPreset::Default => Options::from_preset(2),
            PngPreset::Best => Options::from_preset(4),
            PngPreset::Max => Options::from_preset(6),
        }
    }

    /// PNG プリセットを文字列に変換
    /// * `return` - プリセットの文字列
    pub fn to_string(&self) -> String {
        match self {
            PngPreset::Min => "Min".to_string(),
            PngPreset::Fast => "Fast".to_string(),
            PngPreset::Default => "Default".to_string(),
            PngPreset::Best => "Best".to_string(),
            PngPreset::Max => "Max".to_string(),
        }
    }
}
