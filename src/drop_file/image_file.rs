use std::path::PathBuf;
use getset::Getters;

use crate::app::App;
use crate::optim::Jpeg;

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

        let extension = self.path.extension().unwrap().to_str().unwrap();

        match extension {
            "jpg" | "jpeg" => Jpeg::optimize(&self.path, app)?,
            _ => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported extension")));
            }
        }

        Ok(())
    }
}
