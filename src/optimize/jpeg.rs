use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use crate::optimize::{OptimToken, OptimizeStatus, TEMP_EXTENSION};

/// JPEG 最適化を行う構造体
pub struct Jpeg;

impl Jpeg {
    /// JPEG ファイルを最適化
    /// * `path` - 最適化する JPEG のパス
    /// * `quality` - JPEG 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf, quality: u8, token: OptimToken) -> Result<OptimizeStatus, Box<dyn std::error::Error>> {
        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // ファイルを読み込む
            let file_image = ImageReader::open(&path)?.decode()?;

            // JPEG エンコーダーを作成して最適化
            let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
            encoder.encode_image(&file_image)?;
        }

        // 最適化後のサイズが元のサイズより大きい場合は最適化しない
        let size = path.metadata()?.len() as usize;
        let new_size = buffer.len() as usize;
        if size <= new_size {
            return Ok(OptimizeStatus::Optimized);
        }

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // 一時ファイルを作成して最適化後のデータを保存
        let temp_path = path.with_added_extension(TEMP_EXTENSION);
        std::fs::write(&temp_path, &buffer)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            std::fs::remove_file(&temp_path)?;
            return Ok(OptimizeStatus::Canceled);
        }

        // 一時ファイルを元のファイルに上書き
        std::fs::rename(&temp_path, path)?;

        Ok(OptimizeStatus::Optimized)
    }
}
