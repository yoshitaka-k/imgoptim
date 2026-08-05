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
        if files.len() > 0 {
            let len = files.len();
            let optimizing = files.optimizing_len();
            let optimized = files.optimized_len();
            let error = files.error_len();
            let percent = ((optimized + error) as f32 / len as f32 * 100.0).round() as u32;

            let spacing = ui.spacing().item_spacing.x;

            ui.horizontal(|ui| {
                ui.label("Optimize");

                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}", len), Color32::GREEN, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" files");

                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(text_color(&format!("{}%", percent), Color32::GREEN, None));
                ui.spacing_mut().item_spacing.x = spacing;
                ui.label(" completed");

                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("(");
                ui.spacing_mut().item_spacing.x = spacing;

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
                ui.label(" error)");
            });
        }
    });
}
