use crate::app;

    /// ウィンドウのタイトル
const WINDOW_TITLE: &str = "Img Optim Settings";

// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 160.0;

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
        options, |ui, _class| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label("settings are not available yet");
                ui.label(format!("jpeg quality: {}%", app.jpeg_quality()));
            });

            // ウィンドウの閉じるボタンが押されたら閉じる
            if ui.ctx().input(|input| input.viewport().close_requested()) {
                *settings_window_open = false;
            }
        },
    );
}
