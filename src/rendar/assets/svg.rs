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
