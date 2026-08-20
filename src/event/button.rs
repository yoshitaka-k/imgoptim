use crate::file::open_files;
use crate::optimize::{OptimizeJob, OptimizeStatus};

/// ファイルダイアログを開く
/// * `ui` - UI
/// * `open_dialog` - ファイルダイアログを開くフラグ
pub(crate) fn files_open(ui: &mut egui::Ui, open_dialog: &mut bool) {
    // ファイルダイアログを開くタイミングをずらす
    *open_dialog = true;

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 設定ダイアログを開く
/// * `ui` - UI
/// * `settings_window_open` - 設定ダイアログを開くフラグ
/// * `settings_window_pos` - 設定ダイアログの表示位置
pub(crate) fn setting_open(ui: &mut egui::Ui, settings_window_open: &mut bool, settings_window_pos: &mut Option<egui::Pos2>) {
    // 設定ダイアログを開く
    *settings_window_open = true;

    // 設定ダイアログの表示位置を設定
    *settings_window_pos = ui.ctx().input(|input| {
        input.viewport().outer_rect.map(|rect| rect.min)
    });

    // 再描画を要求
    ui.ctx().request_repaint();
}

/// 最適化を停止（キャンセル）してファイル一覧をクリアする
/// * `files` - ファイル一覧
/// * `optimize_job` - 最適化ジョブ
pub(crate) fn cancel_and_clear(files: &mut open_files::OpenFiles, optimize_job: &mut OptimizeJob) {
    // ファイルをキャンセル
    for file in files.paths() {
        // 待機中か最適化中でない場合はスキップ
        if !matches!(file.status(), OptimizeStatus::Standby | OptimizeStatus::Optimizing) {
            continue;
        }
        // キャンセル ID を追加
        optimize_job.add_canceled_id(*file.id());
    }

    // 最適化を停止（キャンセル）
    optimize_job.stop_running();

    // ファイル一覧をクリア
    files.clear();
}
