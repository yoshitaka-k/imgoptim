mod generated {
    include!(concat!(env!("OUT_DIR"), "/svg_generated.rs"));
}

pub use generated::*;

/// 埋め込み SVG を egui の ImageSource にする
const fn bytes_source(uri: &'static str, bytes: &'static [u8]) -> egui::ImageSource<'static> {
    egui::ImageSource::Bytes {
        uri: std::borrow::Cow::Borrowed(uri),
        bytes: egui::load::Bytes::Static(bytes),
    }
}

/// egui に画像ローダー（SVG 含む）を登録する
pub fn install(ctx: &egui::Context) {
    egui_extras::install_image_loaders(ctx);
}

/// SVG 画像を返す
/// * `image` - SVG const の名前
/// * `size` - サイズ
/// * `tint` - 色
/// * `return` - egui::Image
pub fn image(image: &egui::ImageSource<'static>, size: Option<f32>, tint: Option<egui::Color32>) -> egui::Image {
    egui::Image::new(image).max_height(size.unwrap_or(16.0)).tint(tint.unwrap_or(egui::Color32::WHITE))
}
