#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod file;
mod rendar;
mod optimize;
mod event;

pub use app::App;
pub use file::open_files::OpenFiles;
pub use rendar::Rendar;
pub use optimize::Jpeg;

/// ファイルサイズをフォーマットするマクロ
#[macro_export]
macro_rules! filesize_format {
    ($size:expr) => {
        if $size < 1024 {
            format!("{:.2} B", $size as f64)
        } else if $size < 1024 * 1024 {
            format!("{:.2} KB", $size as f64 / 1024.0)
        } else if $size < 1024 * 1024 * 1024 {
            format!("{:.2} MB", $size as f64 / 1024.0 / 1024.0)
        } else {
            format!("{:.2} GB", $size as f64 / 1024.0 / 1024.0 / 1024.0)
        }
    };
}

/// 最適化時間をフォーマットするマクロ
#[macro_export]
macro_rules! duration_format {
    ($duration:expr) => {
        if $duration < 1000 {
            format!("{:.2} ms", $duration as f64)
        } else if $duration < 60 * 1000 {
            format!("{:.2} s", $duration as f64 / 1000.0)
        } else if $duration < 60 * 60 * 1000 {
            format!("{:.2} m", $duration as f64 / 60.0 / 1000.0)
        } else {
            format!("{:.2} h", $duration as f64 / 60.0 / 60.0 / 1000.0)
        }
    };
}
