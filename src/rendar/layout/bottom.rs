use egui::Color32;

use crate::file::open_files;
use crate::rendar::assets::fonts::text_color;

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn bottom_layout(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
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
            ui.label(text_color(&format!("{:.2}%", files.total_saved_rate()), Color32::GREEN, None));
            ui.spacing_mut().item_spacing.x = spacing;
        });
    });
}
