// リリースビルド時に Windows でコンソールウィンドウを隠す
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(clippy::all, rust_2018_idioms)]

use imgoptim::Rendar;
use imgoptim::App;

/// アプリケーション名
const APP_NAME: &str = "Img Optim";

/// ウィンドウのサイズ
const WINDOW_WIDTH: f32 = 540.0;
const WINDOW_HEIGHT: f32 = 200.0;
const MAX_WINDOW_WIDTH: f32 = 640.0;
const MAX_WINDOW_HEIGHT: f32 = 1024.0;

fn main() -> eframe::Result {
    env_logger::init();

    // アプリケーションのインスタンスを作成
    let app = App::new();

    // ウィンドウのオプション
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_max_inner_size([MAX_WINDOW_WIDTH, MAX_WINDOW_HEIGHT])
            .with_maximize_button(false)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(Rendar::new(cc, app))))
    )
}
