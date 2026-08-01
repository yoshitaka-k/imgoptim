use std::path::PathBuf;
use getset::Getters;

use crate::app::App;
use crate::optim::Jpeg;
use crate::file::extension;

/// 画像ファイルを管理する構造体
#[derive(Getters)]
pub struct ImageFile {
    #[getset(get= "pub")]
    path: PathBuf,

    #[getset(get = "pub")]
    file_name: String,

    #[getset(get = "pub")]
    extension: extension::Extension,

    #[getset(get = "pub")]
    is_optimized: bool,

    #[getset(get = "pub")]
    size: u64,

    #[getset(get = "pub")]
    new_size: u64,

    #[getset(get = "pub")]
    percentage: f32,
}

impl ImageFile {
    /// 新しい ImageFile を作成
    pub fn new(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let ext = path.extension().unwrap();
        let extension = extension::Extension::from_str(ext);

        Self {
            path,
            file_name,
            extension: extension,
            is_optimized: false,
            size: 0,
            new_size: 0,
            percentage: 0.0,
        }
    }

    /// 画像を最適化
    pub fn optimize(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_optimized {
            return Ok(());
        }

        let metadata = self.path.metadata().unwrap();
        self.size = metadata.len();

        match self.extension {
            extension::Extension::Jpeg => {
                self.is_optimized = true;
                Jpeg::optimize(&self.path, app)?;

                let metadata = self.path.metadata().unwrap();
                self.new_size = metadata.len();

                let percentage = (self.size - self.new_size) as f32 / self.size as f32 * 100.0;
                self.percentage = (percentage * 100.0).ceil() / 100.0;
            }
            _ => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported extension")));
            }
        }

        Ok(())
    }
}
