use std::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::HashSet;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::app;
use crate::file::{open_files, image_file};
use crate::file::optimize_status::OptimizeStatus;

/// 最適化ジョブを管理する構造体
pub struct OptimizeJob {
    ctx: egui::Context,

    /// 最適化結果を送信するチャネル
    result_tx: mpsc::Sender<image_file::ImageFile>,
    /// 最適化結果を受信するチャネル
    result_rx: mpsc::Receiver<image_file::ImageFile>,

    /// 最適化実行フラグ
    running: Arc<AtomicBool>,

    /// 最適化実行中のカウント
    running_count: Arc<AtomicUsize>,

    /// キャンセルフラグ
    canceled: Arc<Mutex<HashSet<u64>>>,
}

impl OptimizeJob {
    /// 新しい最適化ジョブを作成
    /// * `ctx` - UI コンテキスト
    /// * `return` - 最適化ジョブ
    pub fn new(ctx: egui::Context) -> Self {
        let (result_tx, result_rx) = mpsc::channel();
        Self {
            ctx,
            result_tx,
            result_rx,
            running: Arc::new(AtomicBool::new(true)),
            running_count: Arc::new(AtomicUsize::new(0)),
            canceled: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// 最適化を実行
    /// * `app` - アプリケーション
    /// * `files` - ファイルリスト
    pub fn run(&self, app: &app::App, files: &mut open_files::OpenFiles) {
        // UI 側一覧にも Optimizing を立ててから clone する
        files.mark_pending_as_optimizing(app);

        // 最適化を開始
        self.start_running();

        // クローンしておく
        let app = app.clone();
        let tx = self.result_tx.clone();
        let ctx = self.ctx.clone();
        let running = Arc::clone(&self.running);
        let canceled = Arc::clone(&self.canceled);

        // 最適化中のファイルを取得
        let mut files: Vec<_> = files.paths()
            .iter()
            .filter(|file| matches!(file.status(), OptimizeStatus::Optimizing))
            .cloned()
            .collect();

        // 開始時にカウントを増やす（スレッド生存中は 1 以上）
        self.running_count.fetch_add(1, Ordering::Relaxed);

        // カウントをクローンしておく
        let running_count = Arc::clone(&self.running_count);

        // 最適化を実行するスレッドを作成
        std::thread::spawn(move || {
            // 最適化を並列実行
            files.par_iter_mut().for_each(|file| {
                // クリアボタンを押されて最適化をキャンセルされていないか確認
                // 1件キャンセルされていないか確認
                if Self::is_canceled(&running, &canceled, file) {
                    // 最適化済みでなければキャンセル状態にする
                    if !matches!(file.status(), OptimizeStatus::Optimized) {
                        // キャンセル状態にする
                        file.set_status(OptimizeStatus::Canceled);
                    }
                } else {
                    // 最適化を実行（並列の各呼び出しで clone が必要）
                    let _ = file.optimize(&app, Arc::clone(&running), Arc::clone(&canceled));
                }

                // 最適化結果を送信
                let _ = tx.send(file.clone());
                // 再描画を要求
                ctx.request_repaint();
            });

            // この最適化のスレッドが終わってからカウントを減らす
            running_count.fetch_sub(1, Ordering::Relaxed);

            // 再描画を要求
            ctx.request_repaint();
        });
    }

    /// 最適化結果を反映
    /// * `files` - ファイルリスト
    pub fn result(&self, files: &mut open_files::OpenFiles) {
        // 最適化結果を受信
        while let Ok(result) = self.result_rx.try_recv() {
            // 最適化結果を反映
            files.apply_result(result);
        }
    }

    /// 最適化を開始
    pub fn start_running(&self) {
        self.running.store(true, Ordering::Relaxed);
    }

    /// 最適化をキャンセル
    pub fn stop_running(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// 最適化実行中かどうか
    /// * `return` - 最適化実行中かどうか
    pub fn is_running_count_zero(&self) -> bool {
        self.running_count.load(Ordering::Relaxed) == 0
    }

    /// 最適化をキャンセル
    /// * `id` - キャンセルするファイルの ID
    pub fn add_canceled_id(&self, id: u64) {
        self.canceled.lock().unwrap().insert(id);
    }

    /// 全体停止、または指定ファイルが1件キャンセル済みか最適化済みかどうか
    /// * `running` - 最適化実行フラグ
    /// * `canceled` - 1件キャンセルされた ID の集合
    /// * `file` - 確認するファイル
    /// * `return` - キャンセルされているかどうか
    fn is_canceled(running: &AtomicBool, canceled: &Mutex<HashSet<u64>>, file: &image_file::ImageFile) -> bool {
        // 全体停止
        if !running.load(Ordering::Relaxed) {
            return true;
        }
        // キャンセル済み
        if canceled.lock().unwrap().contains(file.id()) {
            return true;
        }
        false
    }
}
