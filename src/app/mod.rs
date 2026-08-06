use getset::{Getters, MutGetters};
use serde::{Deserialize, Serialize};

/// アプリケーションを管理する構造体
#[derive(Clone, Getters, MutGetters)]
#[derive(Serialize, Deserialize)]
pub struct App {
    #[getset(get = "pub")]
    #[serde(skip, default = "default_extensions")]
    extensions: Vec<&'static str>,

    #[getset(get = "pub", get_mut = "pub")]
    jpeg_quality: u8,
}

/// デフォルトの拡張子
/// * `return` - デフォルトの拡張子
fn default_extensions() -> Vec<&'static str> {
    vec!["jpg", "jpeg"]
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        Self {
            extensions: default_extensions(),
            jpeg_quality: 80,
        }
    }

    /// 拡張子を文字列に変換
    /// * `return` - 拡張子のベクタ
    pub fn extensions_to_string(&self) -> Vec<String> {
        self.extensions.iter().map(|ext| ext.to_string()).collect()
    }
}
