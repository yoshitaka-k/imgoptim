use crate::app;
use crate::optim::options::PngPreset;
use crate::rendar::assets;
use crate::rendar::assets::{constants, svg};

/// ウィンドウのタイトル
const WINDOW_TITLE: &str = "Keiga Settings";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 180.0;

// 並行処理数の最小値と最大値
const OPTIMIZATION_NUM_MIN: u8 = 1;
const OPTIMIZATION_NUM_MAX: u8 = 6;

// JPEG の品質の最小値と最大値
const JPEG_QUALITY_MIN: u8 = 50;
const JPEG_QUALITY_MAX: u8 = 99;

// 左のスペース
const LEFT_SPACE: f32 = 15.0;

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `settings_window_open` - 設定ウィンドウを開いているかどうか
/// * `window_pos` - ウィンドウの表示位置
pub(crate) fn setting_window(
    ctx: &egui::Context,
    app: &mut app::App,
    settings_window_open: &mut bool,
    window_pos: &mut Option<egui::Pos2>,
) {
    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする
    if let Some(pos) = window_pos.take() {
        options = options.with_position(pos);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("setting_window"),
        options, |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // デフォルトのスペースの幅を避けておく
                let spacing = ui.spacing().item_spacing.x;

                // ボタンの色を設定
                let button_color = assets::button_icon_color(ui);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add(egui::Image::new(svg::SETTINGS).max_height(constants::SETTINGS_ICON_SIZE).tint(button_color));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label("Optimization Settings");
                });

                ui.separator();

                // 並行処理数
                ui.horizontal(|ui| {
                    ui.add_space(LEFT_SPACE);
                    ui.label("Concurrent files:");
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = 282.0;
                        ui.add(egui::Slider::new(app.optimization_num_mut(), OPTIMIZATION_NUM_MIN..=OPTIMIZATION_NUM_MAX));
                    });
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add(egui::Image::new(svg::SETTINGS).max_height(constants::SETTINGS_ICON_SIZE).tint(button_color));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label("Quality Settings");
                });

                ui.separator();

                // JPEG のスライダーを表示
                ui.horizontal(|ui| {
                    ui.add_space(LEFT_SPACE);
                    ui.label("JPEG Quality:");
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = 300.0;
                        ui.add(egui::Slider::new(app.jpeg_quality_mut(), JPEG_QUALITY_MIN..=JPEG_QUALITY_MAX));
                    });
                });

                ui.separator();

                // PNG のプリセットを表示
                ui.horizontal(|ui| {
                    ui.add_space(LEFT_SPACE);
                    ui.label("PNG Preset:");
                    ui.radio_value(app.png_preset_mut(), PngPreset::Min, PngPreset::Min.to_string());
                    ui.radio_value(app.png_preset_mut(), PngPreset::Fast, PngPreset::Fast.to_string());
                    ui.radio_value(app.png_preset_mut(), PngPreset::Default, PngPreset::Default.to_string());
                    ui.radio_value(app.png_preset_mut(), PngPreset::Best, PngPreset::Best.to_string());
                    ui.radio_value(app.png_preset_mut(), PngPreset::Max, PngPreset::Max.to_string());
                });

                ui.separator();
            });

            // ウィンドウの閉じるボタンが押されたら閉じる
            if ctx.input(|input| input.viewport().close_requested()) {
                *settings_window_open = false;
            }
        },
    );
}
