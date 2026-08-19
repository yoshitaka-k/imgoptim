use crate::app;
use crate::optimize::options::PngPreset;
use crate::rendar::setting;

/// 品質を表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, app: &mut app::App) {
    // JPEG のスライダーを表示
    ui.horizontal(|ui| {
        ui.add_space(setting::LEFT_SPACE);
        ui.label("JPEG Quality:");
        ui.scope(|ui| {
            ui.spacing_mut().slider_width = 300.0;
            ui.add(egui::Slider::new(app.jpeg_quality_mut(), setting::JPEG_QUALITY_MIN..=setting::JPEG_QUALITY_MAX));
        });
    });

    ui.separator();

    // PNG のプリセットを表示
    ui.horizontal(|ui| {
        ui.add_space(setting::LEFT_SPACE);
        ui.label("PNG Preset:");
        ui.add_space(8.0);
        ui.radio_value(app.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
        ui.radio_value(app.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
    });
}
