use egui::Color32;

use crate::rendar::fonts::text_color;
use crate::file::open_files;
use crate::file::optimize_status::OptimizeStatus;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, files: &open_files::OpenFiles) {
    // 行間を詰める
    ui.spacing_mut().item_spacing.y = 0.0;

    for (index, path) in files.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(_) => {
                let size = path.size() / 1024;

                match path.status() {
                    OptimizeStatus::None => {
                        ui.horizontal(|ui| {
                            ui.label(path.file_name());
                            ui.label(format!("({} KB)", size));
                        });
                    }
                    OptimizeStatus::Optimizing => {
                        ui.horizontal(|ui| {
                            ui.label(text_color("⏳", Color32::YELLOW, None));
                            ui.label(path.file_name());
                            ui.label(format!("({} KB)", size));
                        });
                    }
                    OptimizeStatus::Optimized => {
                        ui.horizontal(|ui| {
                            ui.label(text_color("✅", Color32::GREEN, None));
                            ui.label(path.file_name());
                            ui.label(format!("({} KB -> {} KB ", size, path.new_size() / 1024));

                            let spacing = ui.spacing().item_spacing.x;

                            // "%"と")"とスペースを0にしてテキストを表示
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label(text_color(&format!("{}%", path.percent()), Color32::GREEN, Some(12.0)));
                            ui.label(")");

                            // スペースを元に戻す
                            ui.spacing_mut().item_spacing.x = spacing;
                        });
                    }
                    OptimizeStatus::Error(e) => {
                        ui.horizontal(|ui| {
                            ui.label(text_color("❌", Color32::RED, None));
                            ui.label(path.file_name());
                            ui.label(text_color(e, Color32::RED, Some(11.0)));
                        });
                    }
                }

                if index < files.paths().len() - 1 {
                    ui.add(egui::Separator::default().spacing(4.0));
                }
            }
            Err(_) => {}
        }
    }
}
