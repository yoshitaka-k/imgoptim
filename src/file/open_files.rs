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

    #[getset(get = "pub", set = "pub")]
    selected_id: Option<u64>,
}

impl OpenFiles {
    /// 新しい OpenFiles を作成
    /// * `return` - OpenFiles のインスタンス
    pub fn new() -> Self {
        Self { paths: vec![], extensions: vec![], selected_id: None }
    }

    /// キャンセルされたファイルのステータスを更新
    /// * `id` - キャンセルされたファイルの ID
    pub fn set_status_canceled(&mut self, id: u64) {
        if let Some(file) = self.paths.iter_mut().find(|f| *f.id() == id) {
            file.set_status(OptimizeStatus::Canceled);
        }
    }

    /// ファイルの数を取得
    /// * `return` - ファイルの数
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// 未処理のファイルの数を取得
    /// * `return` - 未処理のファイルの数
    pub fn pending_len(&self) -> usize {
        self.paths.iter().filter(|p| *p.status() == OptimizeStatus::Standby).count()
    }

    /// 最適化中のファイルの数を取得
    /// * `return` - 最適化中のファイルの数
    pub fn optimizing_len(&self) -> usize {
        self.paths.iter().filter(|p| *p.status() == OptimizeStatus::Optimizing).count()
    }

    /// 最適化済みのファイルの数を取得
    /// * `return` - 最適化済みのファイルの数
    pub fn optimized_len(&self) -> usize {
        self.paths.iter().filter(|p| *p.status() == OptimizeStatus::Optimized).count()
    }

    /// エラーのファイルの数を取得
    /// * `return` - エラーのファイルの数
    pub fn error_len(&self) -> usize {
        self.paths.iter().filter(|p| matches!(p.status(), OptimizeStatus::Error(_))).count()
    }

    /// 総サイズを取得
    /// * `return` - 総サイズ
    pub fn total_size(&self) -> u64 {
        self.paths.iter().map(|p| {
            if *p.status() == OptimizeStatus::Optimized {
                p.size()
            } else {
                &0
            }
        }).sum()
    }

    /// 総新サイズを取得
    /// * `return` - 総新サイズ
    pub fn total_new_size(&self) -> u64 {
        self.paths.iter().map(|p| {
            if *p.status() == OptimizeStatus::Optimized {
                p.new_size()
            } else {
                &0
            }
        }).sum()
    }

    /// 総節約率を取得
    /// * `return` - 総節約率
    pub fn total_saved_rate(&self) -> f32 {
        if self.total_new_size() == 0 {
            return 0.00;
        }

        // 最適化後のファイズによってパーセントを計算
        if self.total_size() >= self.total_new_size() {
            (self.total_size() - self.total_new_size()) as f32 / self.total_size() as f32 * 100.0 * -1.0
        } else {
            (self.total_new_size() - self.total_size()) as f32 / self.total_size() as f32 * 100.0 * 1.0
        }
    }

    /// パスをクリア
    pub fn clear(&mut self) {
        self.paths.clear();
        self.selected_id = None;
    }

    /// パスを追加
    /// * `path` - ドロップされたファイルのパス
    pub fn add_path(&mut self, path: PathBuf) {
        self.find_file(path);
    }

    /// 未処理ファイルを最適化中ステータスへ変更
    pub fn mark_pending_as_optimizing(&mut self) {
        for file in &mut self.paths {
            if *file.status() == OptimizeStatus::Standby {
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
        self.paths.iter().any(|f| *f.status() == OptimizeStatus::Standby)
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
    /// * `path` - ファイルのパス
    /// * `return` - 許可されているかどうか
    fn is_allowed_extension(&self, path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
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
                        "Already optimizing".to_string(),
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
