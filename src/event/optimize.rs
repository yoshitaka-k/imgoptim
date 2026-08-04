use std::sync::mpsc;

use crate::app;
use crate::file::open_files;

pub struct OptimizeJob {
    ctx: egui::Context,

    /// 最適化結果を送信するチャネル
    result_tx: mpsc::Sender<open_files::OpenFiles>,
    /// 最適化結果を受信するチャネル
    result_rx: mpsc::Receiver<open_files::OpenFiles>,
}

impl OptimizeJob {
    /// 新しい最適化ジョブを作成
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
        let mut files = files.clone();
        let tx = self.result_tx.clone();
        let ctx = self.ctx.clone();

        // 最適化を実行するスレッドを作成
        std::thread::spawn(move || {
            let _ = files.optimize(&app);
            let _ = tx.send(files);

            // 再描画を要求
            ctx.request_repaint();
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
        // 最適化結果をファイル単位で反映
        while let Ok(results) = self.result_rx.try_recv() {
            files.apply_results(results);

            // 処理中に追加された未処理があれば続ける
            if files.has_pending() {
                *is_optimizing = true;
            }
        }
    }
}
