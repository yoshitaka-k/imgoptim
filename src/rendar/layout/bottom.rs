use crate::file::open_files;
use crate::event::optimize;
use crate::rendar::assets;
use crate::rendar::assets::{fonts::text_color, svg};

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn bottom_layout(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    optimize_job: &mut optimize::OptimizeJob,
) {
    // 最適化中、最適化済み、エラーのファイル数
    let optimizing_len = files.optimizing_len();
    let optimized_len = files.optimized_len();
    let error_len = files.error_len();

    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 最適化中アイコンの色
    let optimizing_color = assets::optimizing_color(ui);
    // 最適化済みアイコンの色
    let optimized_color = assets::optimized_color(ui);
    // エラーアイコンの色
    let error_color = assets::error_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", optimizing_len), optimizing_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" optimizing,");

        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", optimized_len), optimized_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" optimized,");

        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", error_len), error_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" error");

        ui.separator();

        ui.label("Avg saved rate:");
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{:+.2}%", files.total_saved_rate()), optimized_color, None));
        ui.spacing_mut().item_spacing.x = spacing;

        // 右寄せ
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // ボタンの色を設定
            let button_color = assets::button_icon_color(ui);

            // クリアボタン
            let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(18.0).tint(button_color);
            if ui.button(clear_button).on_hover_text("Cancel and Clear").clicked() {
                // 最適化を停止（キャンセル）
                optimize_job.stop_running();
                // ファイル一覧をクリア
                files.clear();
            }
        });
    });
}
