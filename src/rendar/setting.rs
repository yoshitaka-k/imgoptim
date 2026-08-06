use crate::app;

    /// ウィンドウのタイトル
const WINDOW_TITLE: &str = "Img Optim Settings";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 160.0;

const JPEG_QUALITY_MIN: u8 = 50;
const JPEG_QUALITY_MAX: u8 = 99;

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `settings_window_open` - 設定ウィンドウを開いているかどうか
pub(crate) fn setting_window(ctx: &egui::Context, app: &mut app::App, settings_window_open: &mut bool) {
    let options = egui::ViewportBuilder::default()
        .with_title(WINDOW_TITLE)
        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("setting_window"),
        options, |ctx, _class| {
            egui::CentralPanel::default().show_inside(ctx, |ui| {
                ui.label("Quality Settings");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.add_space(10.0);
                    ui.label("JPEG Quality:");
                    ui.scope(|ui| {
                        ui.spacing_mut().slider_width = 310.0;
                        ui.add(egui::Slider::new(app.jpeg_quality_mut(), JPEG_QUALITY_MIN..=JPEG_QUALITY_MAX));
                    });
                });
            });

            // ウィンドウの閉じるボタンが押されたら閉じる
            if ctx.input(|input| input.viewport().close_requested()) {
                *settings_window_open = false;
            }
        },
    );
}
