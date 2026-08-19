use crate::app;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};
use crate::rendar::setting;
use crate::rendar::setting::{concurrent, quality};

/// 設定ウィンドウを表示
/// * `ctx` - コンテキスト
/// * `settings_window_open` - 設定ウィンドウを開いているかどうか
/// * `window_pos` - ウィンドウの表示位置
pub(crate) fn view(
    ctx: &egui::Context,
    app: &mut app::App,
    settings_window_open: &mut bool,
    window_pos: &mut Option<egui::Pos2>,
) {
    let window_id = egui::ViewportId::from_hash_of(setting::SETTING_WINDOW_ID);

    // 設定ウィンドウのオプションを設定
    let mut options = egui::ViewportBuilder::default()
        .with_title(setting::WINDOW_TITLE)
        .with_inner_size([setting::WINDOW_WIDTH, setting::WINDOW_HEIGHT])
        .with_maximize_button(false)
        .with_resizable(false);

    // ウィンドウの表示位置を指定
    // take()で、取り出して None にする（ボタン押下時だけ位置更新と前面化）
    if let Some(pos) = window_pos.take() {
        options = options.with_position(pos);

        // ウィンドウの位置を更新して前面に出す
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::OuterPosition(pos));
        ctx.send_viewport_cmd_to(window_id, egui::ViewportCommand::Focus);
    }

    // 設定ウィンドウを表示
    ctx.show_viewport_immediate(
        window_id,
        options, |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // デフォルトのスペースの幅を避けておく
                let spacing = ui.spacing().item_spacing.x;

                // ボタンの色を設定
                let button_color = assets::button_icon_color(ui);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add(egui::Image::new(svg::CYCLE).max_height(assets_const::CYCLE_ICON_SIZE).tint(button_color));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label("Concurrent");
                });

                ui.separator();

                // 並行処理数を表示
                concurrent::view(ui, app);

                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    ui.add(egui::Image::new(svg::COMPRESS).max_height(assets_const::COMPRESS_ICON_SIZE).tint(button_color));
                    ui.spacing_mut().item_spacing.x = spacing;
                    ui.label("Quality");
                });

                ui.separator();

                // 品質を表示
                quality::view(ui, app);

                ui.separator();
            });

            // ウィンドウの閉じるボタンが押されたら閉じる
            if ctx.input(|input| input.viewport().close_requested()) {
                *settings_window_open = false;
            }
        },
    );
}
