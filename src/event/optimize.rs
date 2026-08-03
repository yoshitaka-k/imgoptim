use std::sync::mpsc;

use crate::app;
use crate::file::open_files;

pub struct OptimizeJob;

impl OptimizeJob {
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
}
