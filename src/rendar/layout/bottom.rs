use crate::file::open_files;
use crate::optimize::OptimizeJob;
use crate::rendar::assets;
use crate::rendar::assets::{constants, fonts::text_color, svg};

/// 下部ボタンを表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn bottom_layout(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    optimize_job: &mut OptimizeJob,
) {
    // 未処理、最適化中、最適化済み、エラーのファイル数
    files.update_file_length();

    let standby_len = files.standby_len();
    let optimizing_len = files.optimizing_len();
    let optimized_len = files.optimized_len();
    let error_len = files.error_len();

    // デフォルトのスペースの幅を避けておく
    let spacing = ui.spacing().item_spacing.x;

    // 最適化中アイコンの色
    let optimizing_color = assets::optimizing_color(ui);
    // 最適化済みアイコンの色
    let optimized_color = assets::optimized_color(ui);
    // エラーアイコンの色
    let error_color = assets::error_color(ui);

    // 丸アイコンの色
    let circle_color = assets::circle_color(ui);

    ui.horizontal(|ui| {
        if optimizing_len > 0 {
            // 最適化中
            ui.add_space(3.0);
            ui.add(egui::Spinner::new().size(constants::SPINNER_SIZE).color(optimizing_color));
            ui.add_space(3.0);
        } else if optimizing_len == 0 && error_len > 0 {
            // エラー
            ui.add_space(1.0);
            ui.add(egui::Image::new(svg::ERROR).max_height(constants::ERROR_ICON_SIZE).tint(error_color));
            ui.add_space(1.0);
        } else if optimizing_len == 0 && optimized_len > 0 {
            // 最適化済み
            ui.add(egui::Image::new(svg::CHECK).max_height(constants::CHECK_ICON_SIZE).tint(optimized_color));
        } else {
            // 初期状態（最適化中も最適化済みもエラーもない）
            ui.add_space(1.0);
            ui.add(egui::Image::new(svg::CIRCLE).max_height(constants::CIRCLE_ICON_SIZE).tint(circle_color));
            ui.add_space(1.0);
        }

        ui.separator();

        // 未処理
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", standby_len), circle_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" standby,");

        // 最適化中
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", optimizing_len), optimizing_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" optimizing,");

        // 最適化済み
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", optimized_len), optimized_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" optimized,");

        // エラー
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{}", error_len), error_color, None));
        ui.spacing_mut().item_spacing.x = spacing;
        ui.label(" error");

        ui.separator();

        // 平均保存率
        ui.label("Avg saved rate:");
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(text_color(&format!("{:+.2}%", files.total_saved_rate()), optimized_color, None));
        ui.spacing_mut().item_spacing.x = spacing;

        // 右寄せ
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // ボタンの色を設定
            let button_color = assets::button_icon_color(ui);

            // クリアボタン
            let clear_button = egui::Image::new(svg::CLEAR_ALL).max_height(18.0).tint(button_color);
            if ui.button(clear_button).on_hover_text("Cancel and Clear").clicked() {
                // 最適化を停止（キャンセル）
                optimize_job.stop_running();
                // ファイル一覧をクリア
                files.clear();
            }
        });
    });
}
