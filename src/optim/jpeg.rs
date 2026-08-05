use std::path::PathBuf;
use image::ImageReader;
use image::codecs::jpeg::JpegEncoder;
use std::fs::File as FsFile;
use std::io::BufWriter;

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

        // 諸々設定
        let extension = path.extension().unwrap().to_str().unwrap();
        let out_path = path.with_extension(extension);
        let file = FsFile::create(&out_path)?;
        let mut writer = BufWriter::new(file);
        let mut encoder = JpegEncoder::new_with_quality(&mut writer, *app.jpeg_quality());

        // 読み込んでおいたものを最適化して出力
        encoder.encode_image(&file_image)?;

        Ok(())
    }
}
