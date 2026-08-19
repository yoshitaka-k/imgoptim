pub(crate) mod view;
pub(crate) mod concurrent;
pub(crate) mod quality;
pub(crate) mod about;

// ウィンドウのID
pub(crate) const SETTING_WINDOW_ID: &str = "setting_window";

/// ウィンドウのタイトル
pub(crate) const WINDOW_TITLE: &str = "Keiga Settings";

// ウィンドウのサイズ
pub(crate) const WINDOW_WIDTH: f32 = 480.0;
pub(crate) const WINDOW_HEIGHT: f32 = 240.0;

/// スライダーの幅
pub(crate) const CONCURRENT_SLIDER_WIDTH: f32 = 284.0;
pub(crate) const QUALITY_SLIDER_WIDTH: f32 = 330.0;

// 並行処理数の最小値と最大値
pub(crate) const OPTIMIZATION_NUM_MIN: u8 = 3;
pub(crate) const OPTIMIZATION_NUM_MAX: u8 = 8;

pub(crate) const PNG_OPTIMIZATION_NUM_MIN: u8 = 1;
pub(crate) const PNG_OPTIMIZATION_NUM_MAX: u8 = 3;

// JPEG の品質の最小値と最大値
pub(crate) const JPEG_QUALITY_MIN: u8 = 50;
pub(crate) const JPEG_QUALITY_MAX: u8 = 99;
