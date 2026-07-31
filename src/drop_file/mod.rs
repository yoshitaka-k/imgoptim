mod image_file;

use std::fs;
use std::path::PathBuf;
use getset::{Getters, Setters};

use crate::app::App;
use crate::drop_file::image_file::ImageFile;

/// ドロップされたファイルを管理する構造体
#[derive(Getters, Setters)]
pub struct DropFile {
    #[getset(get = "pub")]
    paths: Vec<ImageFile>,

    #[getset(set = "pub")]
    extensions: Vec<&'static str>,
}

impl DropFile {
    /// 新しい DropFile を作成
    /// * `return` - DropFile のインスタンス
    pub fn new() -> Self {
        Self { paths: vec![], extensions: vec![] }
    }

    /// パスをクリア
    pub fn clear(&mut self) {
        self.paths.clear();
    }

    /// パスを追加
    /// * `path` - ドロップされたファイルのパス
    pub fn add_path(&mut self, path: PathBuf) {
        self.find_file(path);
    }

    /// ファイルを最適化
    /// * `app` - アプリケーションの設定
    pub fn optimize(&self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        for file in self.paths.iter() {
            file.optimize(app)?;
        }

        Ok(())
    }

    /// ファイルの拡張子が許可されているかどうかを確認
    /// * `file` - ファイルのパス
    /// * `return` - 許可されているかどうか
    fn is_allowed_extension(&self, file: &PathBuf) -> bool {
        if let Some(ext) = file.extension() {
            if let Some(ext) = ext.to_str() {
                if self.extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                    return true;
                }
            }
        }
        false
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - ファイルのパス
    fn find_file(&mut self, path: PathBuf) {
        let metadata = path.metadata().expect("metadata call failed");
        if metadata.is_file() {
            if self.is_allowed_extension(&path) {
                self.paths.push(ImageFile::new(path));
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(path).expect("read_dir call failed") {
                let entry = entry.expect("entry call failed");
                self.find_file(entry.path());
            }
        }
    }
}
