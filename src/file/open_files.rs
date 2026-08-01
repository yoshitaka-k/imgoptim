use std::fs;
use std::path::PathBuf;
use getset::{Getters, Setters};

use crate::app::App;
use crate::file::image_file;

/// ドロップされたファイルを管理する構造体
#[derive(Getters, Setters)]
pub struct OpenFiles {
    #[getset(get = "pub")]
    paths: Vec<image_file::ImageFile>,

    #[getset(set = "pub")]
    extensions: Vec<&'static str>,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
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
    pub fn optimize(&mut self, app: &App) -> Result<(), Box<dyn std::error::Error>> {
        for file in self.paths.iter_mut() {
            match file.optimize(app) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("optimize failed: {}", e);
                }
            }
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
                self.paths.push(image_file::ImageFile::new(path));
            }
        } else if metadata.is_dir() {
            for entry in fs::read_dir(path).expect("read_dir call failed") {
                let entry = entry.expect("entry call failed");
                self.find_file(entry.path());
            }
        }
    }
}
