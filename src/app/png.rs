use serde::{Deserialize, Serialize};

/// PNG 圧縮タイプ
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngCompression {
    Default,
    Fast,
    Best,
    Uncompressed,
    Level(u8),
}

/// PNG フィルター
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngFilter {
    NoFilter,
    Sub,
    Up,
    Avg,
    Paeth,
    Adaptive,
}
