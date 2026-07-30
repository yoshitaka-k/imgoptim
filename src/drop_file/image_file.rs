use std::io::BufWriter;
use std::path::PathBuf;
use std::fs::File as FsFile;
use getset::Getters;
use image::codecs::jpeg::JpegEncoder;
use image::ImageReader;

use crate::app::App;

/// 画像ファイルを管理する構造体
#[derive(Getters)]
pub struct ImageFile {
    #[getset(get= "pub")]
    path: PathBuf,
}

impl ImageFile {
    /// 新しい ImageFile を作成
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// 画像を最適化
    pub fn optimize(&self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        println!("Optimize: {}", self.path.display());

        let file_image = ImageReader::open(&self.path)?.decode()?;
        let extension = self.path.extension().unwrap().to_str().unwrap();

        match extension {
            "jpg" | "jpeg" => {
                println!("Optimize: {}", self.path.display());

                let out_path = self.path.with_extension(extension);
                let file = FsFile::create(&out_path)?;
                let mut writer = BufWriter::new(file);
                let mut encoder = JpegEncoder::new_with_quality(&mut writer, *app.jpeg_quality());
                encoder.encode_image(&file_image)?;
            }
            _ => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported extension")));
            }
        }

        Ok(())
    }
}
