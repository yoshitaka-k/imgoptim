use std::path::PathBuf;
use egui::Color32;

use crate::rendar::assets::{fonts::text_color, svg};
use crate::file::{open_files, optimize_status::OptimizeStatus};

enum FileListAction {
    Click { id: u64 },
    DoubleClick { path: PathBuf },
}

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
pub(crate) fn file_list(ui: &mut egui::Ui, files: &mut open_files::OpenFiles) {
    let width = ui.available_width();
    let mut pending_action: Vec<FileListAction> = Vec::new();
    let clone_files = files.clone();

    // 行間を詰める
    ui.spacing_mut().item_spacing.y = 0.0;

    for (index, path) in clone_files.paths().iter().enumerate() {
        let size = path.size() / 1024;

        let is_selected = *files.selected_id() == Some(*path.id());
        let fill_color = if is_selected {
            Color32::from_rgba_unmultiplied(255, 255, 255, 50)
        } else {
            Color32::from_rgba_unmultiplied(255, 255, 255, 0)
        };

        egui::Frame::new()
            .fill(fill_color)
            .show(ui, |ui|
        {
            ui.set_min_width(width);

            let response = ui.horizontal(|ui| {
                match path.status() {
                    OptimizeStatus::Standby => {
                        ui.label(path.file_name());
                        ui.label(format!("({} KB)", size));
                    }
                    OptimizeStatus::Optimizing => {
                        ui.add(egui::Image::new(svg::HOURGLASS_BOTTOM).max_height(12.0).tint(Color32::YELLOW));
                        ui.label(path.file_name());
                        ui.label(format!("({} KB)", size));
                    }
                    OptimizeStatus::Optimized => {
                        ui.add(egui::Image::new(svg::CHECK).max_height(14.0).tint(Color32::GREEN));
                        ui.label(path.file_name());
                        ui.label(format!("({} KB -> {} KB {}%)", size, path.new_size() / 1024, path.percent()));
                    }
                    OptimizeStatus::Error(e) => {
                        ui.add(egui::Image::new(svg::ERROR).max_height(14.0).tint(Color32::RED));
                        ui.label(path.file_name());
                        ui.label(text_color(e, Color32::RED, Some(11.0)));
                    }
                }
            }).response.interact(egui::Sense::click());

            if response.clicked() {
                pending_action.push(FileListAction::Click { id: *path.id() });
            }

            if response.double_clicked() {
                pending_action.push(FileListAction::DoubleClick { path: path.path().clone() });
            }
        });

        if index < files.paths().len() - 1 {
            ui.add(egui::Separator::default().spacing(4.0));
        }
    }

    // クリックアクションを処理
    for action in pending_action {
        match action {
            FileListAction::Click { id } => {
                files.set_selected_id(Some(id));
            }
            FileListAction::DoubleClick { path } => {
                // Finder でファイルを選択表示する
                if let Err(err) = std::process::Command::new("open")
                    .args(["-R", path.to_str().unwrap()])
                    .status()
                {
                    eprintln!("Error revealing file: {}", err);
                }
            },
        };
    }
}
