use std::{env, fs::File, path::Path};

/// Windows 向けに ICO を生成し、実行ファイルへ埋め込む
/// * `manifest_path` - CARGO_MANIFEST_DIR
/// * `out_dir` - OUT_DIR
pub(crate) fn embed_windows_icon(manifest_path: &Path, out_dir: &str) {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    let png_path = manifest_path.join("assets/icon.png");
    println!("cargo:rerun-if-changed={}", png_path.display());

    let ico_path = Path::new(out_dir).join("icon.ico");
    write_ico(&png_path, &ico_path);

    winresource::WindowsResource::new()
        .set_icon(ico_path.to_str().expect("invalid ico path"))
        .compile()
        .expect("failed to compile windows resources");
}

/// PNG から複数サイズの ICO を書き出す
/// * `png_path` - ソース PNG
/// * `ico_path` - 出力 ICO
fn write_ico(png_path: &Path, ico_path: &Path) {
    let imaged = image::open(png_path).expect("failed to open assets/icon.png");
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in [256, 128, 64, 48, 32, 16_u32] {
        let resized = imaged.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let rgba = resized.to_rgba8();
        let icon_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).expect("failed to encode ico entry"));
    }

    let file = File::create(ico_path).expect("failed to create icon.ico");
    icon_dir.write(file).expect("failed to write icon.ico");
}
