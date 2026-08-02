use std::path::PathBuf;
use getset::Getters;

use crate::app::App;
use crate::optim::Jpeg;
use crate::file::extension;

/// 画像ファイルを管理する構造体
#[derive(Clone, Getters)]
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
    percent: f32,
}

impl ImageFile {
    /// 新しい ImageFile を作成
    /// * `path` - ファイルのパス
    /// * `return` - ImageFile のインスタンス
    pub fn new(path: PathBuf) -> Self {
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let ext = path.extension().unwrap();
        let extension = extension::Extension::from_str(ext);
        let size = path.metadata().unwrap().len();

        Self {
            path,
            file_name,
            extension: extension,
            is_optimized: false,
            size: size,
            new_size: 0,
            percent: 0.0,
        }
    }

    /// jpeg ファイルの最適化処理とファイルサイズの更新
    /// * `app` - アプリケーションの設定
    /// * `return` - 最適化の結果
    fn jpeg_optimize(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        self.is_optimized = true;

        // 最適化を実行
        Jpeg::optimize(&self.path, app)?;

        // 最適化後のファイル情報
        let metadata = self.path.metadata().unwrap();
        self.new_size = metadata.len();

        if self.size > 0 && self.size >= self.new_size {
            let percent = (self.size - self.new_size) as f32 / self.size as f32 * 100.0;
            self.percent = (percent * 100.0).ceil() / 100.0;
        } else {
            self.percent = 0.0;
        }

        Ok(())
    }

    /// 画像を最適化
    /// * `app` - アプリケーションの設定
    /// * `return` - 最適化の結果
    pub fn optimize(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_optimized {
            return Ok(());
        }

        match self.extension {
            extension::Extension::Jpeg => {
                self.jpeg_optimize(app)?;
            }
            _ => {
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported extension")));
            }
        }

        Ok(())
    }
}
