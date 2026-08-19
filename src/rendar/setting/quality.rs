use crate::app;
use crate::optimize::options::PngPreset;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};
use crate::rendar::setting;

/// 品質を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add(egui::Image::new(svg::COMPRESS).max_height(assets_const::COMPRESS_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("Quality");
    });

    ui.separator();

    // JPEG のスライダーを表示
    ui.horizontal(|ui| {
        ui.label("JPEG Quality:");
        ui.scope(|ui| {
            ui.spacing_mut().slider_width = setting::QUALITY_SLIDER_WIDTH;
            ui.add(egui::Slider::new(app.jpeg_quality_mut(), setting::JPEG_QUALITY_MIN..=setting::JPEG_QUALITY_MAX));
        });
    });

    ui.separator();

    // PNG のプリセットを表示
    ui.horizontal(|ui| {
        ui.label("PNG Preset:");
        ui.add_space(8.0);
        ui.radio_value(app.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
    });
}
