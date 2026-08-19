use crate::rendar::assets::{svg, constants};

/// エラーモーダルを表示
/// * `show_modal` - モーダルを表示するかどうか
/// * `ctx` - コンテキスト
/// * `error` - エラー
pub(crate) fn error(show_modal: &mut bool, ctx: &egui::Context, error: &mut Option<Box<dyn std::error::Error>>) {
    if error.is_none() {
        return;
    }

    // モーダルを表示
    let modal = egui::Modal::new(egui::Id::new("error")).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Image::new(svg::ERROR).max_height(constants::MODAL_ERROR_ICON_SIZE).tint(egui::Color32::RED));
            ui.heading("An error occurred");
        });

        if let Some(error) = error {
            ui.label(error.to_string());
        }
    });

    // モーダルを閉じたらモーダルを非表示にする
    if modal.should_close() {
        *show_modal = false;
        *error = None;
    }
}
