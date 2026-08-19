use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use crate::optimize::{OptimToken, OptimizeStatus};

/// JPEG 最適化を行う構造体
pub struct Jpeg;

impl Jpeg {
    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `quality` - JPEG 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf, quality: u8, token: OptimToken) -> Result<OptimizeStatus, Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let file_image = ImageReader::open(&path)?.decode()?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // JPEG エンコーダーを作成
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
            encoder.encode_image(&file_image)?;
        }

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // ファイルを上書き保存
        std::fs::write(path, &buffer)?;

        Ok(OptimizeStatus::Optimized)
    }
}
