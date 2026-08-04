use std::fs;
use std::path::PathBuf;
use getset::{Getters, Setters};

use crate::file::image_file;
use crate::file::optimize_status::OptimizeStatus;

/// ドロップされたファイルを管理する構造体
#[derive(Clone, PartialEq, Getters, Setters)]
pub struct OpenFiles {
    #[getset(get = "pub")]
    paths: Vec<image_file::ImageFile>,

    #[getset(set = "pub")]
    extensions: Vec<String>,
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

    /// 未処理ファイルを最適化中ステータスへ変更
    pub fn mark_pending_as_optimizing(&mut self) {
        for file in &mut self.paths {
            if *file.status() == OptimizeStatus::None {
                file.set_status(OptimizeStatus::Optimizing);
            }
        }
    }

    /// 最適化中のファイルがあるかどうか
    /// * `return` - 最適化中のファイルがあるかどうか
    pub fn has_optimizing(&self) -> bool {
        self.paths.iter().any(|f| *f.status() == OptimizeStatus::Optimizing)
    }

    /// 未処理ファイルがあるかどうか
    /// * `return` - 未処理ファイルがあるかどうか
    pub fn has_pending(&self) -> bool {
        self.paths.iter().any(|f| *f.status() == OptimizeStatus::None)
    }

    /// 最適化結果を既存の一覧へ反映
    /// * `results` - 最適化済みのファイル
    pub fn apply_result(&mut self, result: image_file::ImageFile) {
        // 既存のファイル一覧から ID が一致するファイルを検索
        if let Some(file) = self.paths.iter_mut().find(|f| f.id() == result.id()) {
            *file = result;
        }
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

    /// ファイルが最適化中かどうかを確認
    /// * `path` - ファイルのパス
    /// * `return` - 最適化中かどうか
    fn is_optimizing(&self, path: &PathBuf) -> bool {
        self.paths.iter().any(|f| f.path() == path && *f.status() == OptimizeStatus::Optimizing)
    }

    /// ファイルを検索
    /// * `path` - ドロップされたファイルのパス
    /// * `return` - ファイルのパス
    fn find_file(&mut self, path: PathBuf) {
        let metadata = path.metadata().expect("metadata call failed");
        if metadata.is_file() {
            if self.is_allowed_extension(&path) {
                // 同じパスが最適化中なら、新規行をエラーで追加
                if self.is_optimizing(&path) {
                    let mut image_file = image_file::ImageFile::new(path);
                    image_file.set_status(OptimizeStatus::Error(
                        "同じファイルが最適化中です".to_string(),
                    ));
                    self.paths.push(image_file);
                    return;
                }

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
