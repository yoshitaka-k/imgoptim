use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};
use oxipng::{Options, StripChunks};

use crate::optim::options::PngPreset;

const DEFAULT_JPEG_QUALITY: u8 = 80;
const DEFAULT_PNG_PRESET: PngPreset = PngPreset::Best;

/// アプリケーションを管理する構造体
#[derive(Clone, Getters, MutGetters)]
#[derive(Serialize, Deserialize)]
pub struct App {
    #[getset(get = "pub")]
    #[serde(skip, default = "default_extensions")]
    extensions: Vec<&'static str>,

    #[getset(get = "pub", get_mut = "pub")]
    jpeg_quality: u8,

    #[getset(get = "pub", get_mut = "pub")]
    png_preset: PngPreset,
}

/// デフォルトの拡張子
/// * `return` - デフォルトの拡張子
fn default_extensions() -> Vec<&'static str> {
    vec!["jpg", "jpeg", "png"]
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        Self {
            extensions: default_extensions(),
            jpeg_quality: DEFAULT_JPEG_QUALITY,
            png_preset: DEFAULT_PNG_PRESET,
        }
    }

    /// 拡張子を文字列に変換
    /// * `return` - 拡張子のベクタ
    pub fn extensions_to_string(&self) -> Vec<String> {
        self.extensions.iter().map(|ext| ext.to_string()).collect()
    }

    /// PNG 最適化オプションを作成
    /// * `return` - 最適化オプション
    pub fn png_options(&self) -> Options {
        let mut options = self.png_preset().to_options();
        options.strip = StripChunks::Safe;
        options
    }
}
