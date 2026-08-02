use getset::Getters;

/// アプリケーションを管理する構造体
#[derive(Clone,Getters)]
pub struct App {
    #[getset(get = "pub")]
    extensions: Vec<&'static str>,

    #[getset(get = "pub")]
    jpeg_quality: u8,
}

impl App {
    /// 新しい App を作成
    /// * `return` - App のインスタンス
    pub fn new() -> Self {
        let extensions = vec![
            "jpg", "jpeg", "png", "gif", "bmp",
        ];

        Self {
            extensions: extensions,
            jpeg_quality: 80,
        }
    }

    /// 拡張子を文字列に変換
    /// * `return` - 拡張子のベクタ
    pub fn extensions_to_string(&self) -> Vec<String> {
        self.extensions.iter().map(|ext| ext.to_string()).collect()
    }
}
