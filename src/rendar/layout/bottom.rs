use egui::Color32;

use crate::file::open_files;
use crate::event::optimize;
use crate::rendar::assets::{fonts::text_color, svg};

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn bottom_layout(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    optimize_job: &mut optimize::OptimizeJob,
) {
    ui.separator();

    // 左寄せ
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        let optimizing = files.optimizing_len();
        let optimized = files.optimized_len();
        let error = files.error_len();

        let spacing = ui.spacing().item_spacing.x;

        ui.horizontal(|ui| {
            ui.add_space(2.0);

            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(text_color(&format!("{}", optimizing), Color32::YELLOW, None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(" optimizing,");

            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(text_color(&format!("{}", optimized), Color32::GREEN, None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(" optimized,");

            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(text_color(&format!("{}", error), Color32::RED, None));
            ui.spacing_mut().item_spacing.x = spacing;
            ui.label(" error");

            ui.separator();

            ui.label("Avg saved rate:");
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(text_color(&format!("{:+.2}%", files.total_saved_rate()), Color32::GREEN, None));
            ui.spacing_mut().item_spacing.x = spacing;

            // 右寄せ
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // クリアボタン
                let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(18.0).tint(Color32::WHITE);
                if ui.button(clear_button).on_hover_text("Cancel and Clear").clicked() {
                    // 最適化を停止（キャンセル）
                    optimize_job.stop_running();
                    // ファイル一覧をクリア
                    files.clear();
                }
            });
        });
    });
}
