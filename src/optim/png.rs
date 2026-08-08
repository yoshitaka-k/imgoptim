use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use image::{ImageReader, ImageEncoder, ExtendedColorType};
use image::codecs::png::PngEncoder;

use crate::app::App;

/// PNG 最適化を行う構造体
pub struct Png;

impl Png {
    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `app` - アプリケーションの設定
    /// * `running` - 最適化中かどうか
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf, app: &App, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let file_image = ImageReader::open(&path)?.decode()?;

        // 最適化中止された場合は処理を中断
        if !running.load(Ordering::Relaxed) {
            return Ok(());
        }

        // メモリ上にバッファを作成して最適化
        let mut buffer = Vec::new();
        {
            // 画像を RGBA8 に変換
            let data = file_image.to_rgba8();
            // PNG エンコーダーを作成
            let encoder = PngEncoder::new_with_quality(&mut buffer, app.png_compression(), app.png_filter());
            encoder.write_image(&data.as_raw(), data.width(), data.height(), ExtendedColorType::Rgba8)?;
        }

        // 最適化中止された場合は処理を中断
        if !running.load(Ordering::Relaxed) {
            return Ok(());
        }

        // ファイルを上書き保存
        std::fs::write(path, buffer)?;

        Ok(())
    }
}
