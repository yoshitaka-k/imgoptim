use crate::app;
use crate::rendar::assets;
use crate::rendar::assets::{constants as assets_const, svg};

/// バージョンを表示
/// * `ui` - UI
/// * `app` - アプリ
pub(crate) fn view(ui: &mut egui::Ui, _app: &mut app::App) {
    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // ボタンの色を設定
    let icon_color = assets::icon_color(ui);

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add(egui::Image::new(svg::INFO).max_height(assets_const::INFO_ICON_SIZE).tint(icon_color));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label("About");
    });

    ui.separator();

    ui.add(egui::Image::new(assets::APP_ICON).max_height(assets_const::APP_ICON_SIZE));

    ui.label(format!("Keiga v{}", env!("CARGO_PKG_VERSION")));
    ui.label(env!("CARGO_PKG_DESCRIPTION"));

    ui.label("");

    ui.label(format!("License: {}", env!("CARGO_PKG_LICENSE")));

    ui.horizontal(|ui| {
        ui.label("Repository:");
        ui.hyperlink_to(env!("CARGO_PKG_REPOSITORY"), format!("https://github.com/{}", env!("CARGO_PKG_REPOSITORY")));
    });
}
