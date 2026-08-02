use crate::file::open_files;
use crate::file::image_file;

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, files: &open_files::OpenFiles) {
    for (index, path) in files.paths().iter().enumerate() {
        match path.path().metadata() {
            Ok(_) => {
                match path.status() {
                    image_file::OptimizeStatus::None => {
                        ui.label(format!(
                            "{} ({} KB)",
                            path.file_name(),
                            path.size() / 1024,
                        ));
                    }
                    image_file::OptimizeStatus::Optimizing => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("⏳").color(egui::Color32::YELLOW));
                            ui.label(path.file_name());
                            ui.label(format!("({} KB)", path.size() / 1024));
                        });
                    }
                    image_file::OptimizeStatus::Optimized => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("✅").color(egui::Color32::GREEN));
                            ui.label(path.file_name());
                            ui.label(format!("({} KB -> {} KB) {}%", path.size() / 1024, path.new_size() / 1024, path.percent()));
                        });
                    }
                    image_file::OptimizeStatus::Error(e) => {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("❌").color(egui::Color32::RED));
                            ui.label(path.file_name());
                            ui.label(format!("({} KB) {}", path.size() / 1024, e));
                        });
                    }
                }

                if index < files.paths().len() - 1 {
                    ui.separator();
                }
            }
            Err(_) => {}
        }
    }

    if files.paths().len() > 0 {
        ui.separator();
    }

    ui.add_space(20.0);
}
