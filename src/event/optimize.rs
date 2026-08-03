use std::sync::mpsc;

use crate::app;
use crate::file::open_files;

pub struct OptimizeJob;

impl OptimizeJob {
    /// 最適化を実行
    /// * `app` - アプリケーション
    /// * `files` - ファイルリスト
    /// * `result_tx` - 最適化結果を送信するチャネル
    /// * `ctx` - コンテキスト
    pub fn run(
        app: app::App,
        files: open_files::OpenFiles,
        result_tx: mpsc::Sender<open_files::OpenFiles>,
        ctx: &egui::Context
    ) {
        let app = app.clone();
        let mut files = files.clone();
        let tx = result_tx.clone();
        let ctx = ctx.clone();

        // 最適化を実行するスレッドを作成
        std::thread::spawn(move || {
            let _ = files.optimize(&app);
            let _ = tx.send(files);

            // 再描画を要求
            ctx.request_repaint();
        });
    }

    /// 最適化結果を反映
    /// * `result_rx` - 最適化結果を受信するチャネル
    /// * `files` - ファイルリスト
    /// * `is_optimizing` - 最適化中かどうか
    pub fn result(
        result_rx: &mut mpsc::Receiver<open_files::OpenFiles>,
        files: &mut open_files::OpenFiles,
        is_optimizing: &mut bool,
    ) {
        // 最適化結果をファイル単位で反映
        while let Ok(results) = result_rx.try_recv() {
            files.apply_results(results);

            // 処理中に追加された未処理があれば続ける
            if files.has_pending() {
                *is_optimizing = true;
            }
        }
    }
}
