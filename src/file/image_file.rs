use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering, AtomicBool};
use getset::{Getters, Setters};

use crate::app::App;
use crate::optim::Jpeg;
use crate::file::extension;
use crate::file::optimize_status::OptimizeStatus;

/// ImageFile の一意な ID を発行するカウンタ
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// 画像ファイルを管理する構造体
#[derive(Clone, PartialEq, Getters, Setters)]
pub struct ImageFile {
    #[getset(get = "pub")]
    id: u64,

    #[getset(get= "pub")]
    path: PathBuf,

    #[getset(get = "pub")]
    file_name: String,

    #[getset(get = "pub")]
    extension: extension::Extension,

    #[getset(get = "pub", set = "pub")]
    status: OptimizeStatus,

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
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let ext = path.extension().unwrap();
        let extension = extension::Extension::from_str(ext);
        let size = path.metadata().unwrap().len();

        Self {
            id,
            path,
            file_name,
            extension,
            status: OptimizeStatus::Standby,
            size,
            new_size: 0,
            percent: 0.0,
        }
    }

    /// jpeg ファイルの最適化処理とファイルサイズの更新
    /// * `app` - アプリケーションの設定
    /// * `return` - 最適化の結果
    fn jpeg_optimize(&mut self, app: &App, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        // 最適化を実行
        Jpeg::optimize(&self.path, app, running)?;

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
    pub fn optimize(&mut self, app: &App, running: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
        // 完了済み・エラー済みは再実行しない
        if matches!(
            self.status,
            OptimizeStatus::Optimized | OptimizeStatus::Error(_)
        ) {
            return Ok(());
        }

        // 最適化中にする
        self.status = OptimizeStatus::Optimizing;

        // 最適化を実行
        let result = match self.extension {
            // jpeg ファイルの最適化
            extension::Extension::Jpeg => self.jpeg_optimize(app, Arc::clone(&running)),
            // サポートしていないファイル形式
            _ => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported extension",
            )) as Box<dyn std::error::Error>),
        };

        match result {
            Ok(()) => {
                // 最適化中止された場合は処理を中断
                if !running.load(Ordering::Relaxed) {
                    self.status = OptimizeStatus::Canceled;
                    return Ok(());
                }

                self.status = OptimizeStatus::Optimized;
                Ok(())
            }
            Err(e) => {
                self.status = OptimizeStatus::Error(e.to_string());
                Err(e)
            }
        }
    }
}
