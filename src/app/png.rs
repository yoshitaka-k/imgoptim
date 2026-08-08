use serde::{Deserialize, Serialize};

/// PNG 圧縮タイプ
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum PngCompression {
    Uncompressed,
    Fast,
    Default,
    Best,
    Max,
    Level(u8),
}
