use std::sync::mpsc;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::app;
use crate::file::open_files;
use crate::file::image_file;

pub struct OptimizeJob {
    ctx: egui::Context,

    /// 最適化結果を送信するチャネル
    result_tx: mpsc::Sender<image_file::ImageFile>,
    /// 最適化結果を受信するチャネル
    result_rx: mpsc::Receiver<image_file::ImageFile>,
}

impl OptimizeJob {
    /// 新しい最適化ジョブを作成
    /// * `ctx` - UI コンテキスト
    /// * `return` - 最適化ジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self { ctx, result_tx, result_rx }
    }

    /// 最適化を実行
    /// * `app` - アプリケーション
    /// * `files` - ファイルリスト
    pub fn run(
        &self,
        app: &app::App,
        files: &mut open_files::OpenFiles,
    ) {
        // UI 側一覧にも Optimizing を立ててから clone する
        files.mark_pending_as_optimizing();

        // クローンしておく
        let app = app.clone();
        let mut files = files.paths().clone();
        let tx = self.result_tx.clone();
        let ctx = self.ctx.clone();

        // 最適化を実行するスレッドを作成
        std::thread::spawn(move || {
            // 最適化を並列実行
            files.par_iter_mut().for_each(|file| {
                // 最適化を実行
                let _ = file.optimize(&app);
                // 最適化結果を送信
                let _ = tx.send(file.clone());
                // 再描画を要求
                ctx.request_repaint();
            });
        });
    }

    /// 最適化結果を反映
    /// * `files` - ファイルリスト
    /// * `is_optimizing` - 最適化中かどうか
    pub fn result(
        &self,
        files: &mut open_files::OpenFiles,
        is_optimizing: &mut bool,
    ) {
        // 最適化結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // 最適化結果を反映
            files.apply_result(result);

            // 処理中に追加された未処理があれば続ける
            if files.has_pending() {
                *is_optimizing = true;
            }
        }
    }
}
