use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;

use crate::app::App;

/// JPEG 最適化を行う構造体
pub struct Jpeg;

impl Jpeg {
    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `app` - アプリケーションの設定
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let file_image = ImageReader::open(&path)?.decode()?;

        let mut buffer = Vec::new();
        {
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, *app.jpeg_quality());
            encoder.encode_image(&file_image)?;
        }
        std::fs::write(path, &buffer)?;

        Ok(())
    }
}
