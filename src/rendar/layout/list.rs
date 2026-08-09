use std::{ops::Range, path::PathBuf};
use egui::{Color32, Sense};

use crate::file::{open_files, optimize_status::OptimizeStatus};
use crate::rendar::assets;
use crate::rendar::assets::{constants, fonts::text_color, svg};

/// ファイル一覧のアクション
enum FileListAction {
    Hover { id: u64 },
    Click { id: u64 },
    DoubleClick { path: PathBuf },
}

/// show_rows 用の高さ
/// * `ui` - UI
/// * `return` - 行高
pub(crate) fn row_height(ui: &egui::Ui) -> f32 {
    ui.text_style_height(&egui::TextStyle::Body).max(constants::CHECK_ICON_SIZE) + constants::SEPARATOR_HEIGHT
}

/// ファイル一覧を表示
/// * `ui` - UI
/// * `files` - ドロップされたファイル
/// * `row_range` - 表示する行の範囲
/// * `row_height` - show_rows に渡したのと同じ値
/// * `return` - いずれかの行がクリックされたかどうか
pub(crate) fn file_list(
    ui: &mut egui::Ui,
    files: &mut open_files::OpenFiles,
    row_range: Range<usize>,
    row_height: f32
) -> bool {
    let mut pending_action: Vec<FileListAction> = Vec::new();
    let mut row_clicked = false;
    let width = ui.available_width();
    let clone_files = files.clone();
    let total = clone_files.paths().len();
    let row_spacing = ui.spacing().item_spacing.y;

    // リストを表示
    for index in row_range {
        let Some(path) = clone_files.paths().get(index) else {
            break;
        };

        // 表示するファイルサイズを計算
        let size = path.size() / 1024;
        let new_size = path.new_size() / 1024;

        // 高さがズレると赤くチラつくので予めサイズ確保
        // 行のクリックイベントを受け取るために Sense::click() を指定
        let (row_rect, response) = ui.allocate_exact_size(
            egui::vec2(width, row_height),
            Sense::click(),
        );

        // 最適化中アイコンの色
        let optimizing_color = assets::optimizing_color(ui);
        // 最適化済みアイコンの色
        let optimized_color = assets::optimized_color(ui);
        // エラーアイコンの色
        let error_color = assets::error_color(ui);

        // 交互に背景色
        if index % 2 == 0 {
            ui.painter().rect_filled(row_rect, 1.0, if ui.ctx().global_style().visuals.dark_mode {
                Color32::from_rgba_unmultiplied(255, 255, 255, 5)
            } else {
                Color32::from_rgba_unmultiplied(0, 0, 0, 10)
            });
        }

        // 選択されている場合は背景を表示
        if *files.selected_id() == Some(*path.id()) {
            ui.painter().rect_filled(row_rect, 1.0, if ui.ctx().global_style().visuals.dark_mode {
                Color32::from_rgba_unmultiplied(255, 255, 255, 50)
            } else {
                Color32::from_rgba_unmultiplied(0, 0, 0, 50)
            });
        }

        // コンテンツを表示
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(row_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        |ui| {
            ui.add_space(5.0);

            // 最適化ステータスに応じて表示
            match path.status() {
                OptimizeStatus::Standby => {
                    ui.label(path.file_name());
                    ui.label(format!("({}KB)", size));
                }
                OptimizeStatus::Optimizing => {
                    ui.add(egui::Image::new(svg::HOURGLASS_BOTTOM).max_height(constants::HOURGLASS_ICON_SIZE).tint(optimizing_color));
                    ui.label(path.file_name());
                    ui.label(format!("({}KB)", size));
                }
                OptimizeStatus::Optimized => {
                    ui.add(egui::Image::new(svg::CHECK).max_height(constants::CHECK_ICON_SIZE).tint(optimized_color));
                    ui.label(path.file_name());
                    ui.label(format!("({}KB -> {}KB | {:+.2}%)", size, new_size, path.percent()));
                }
                OptimizeStatus::Error(e) => {
                    ui.add(egui::Image::new(svg::ERROR).max_height(constants::ERROR_ICON_SIZE).tint(error_color));
                    ui.label(path.file_name());
                    ui.label(text_color(e, error_color, Some(11.0)));
                }
                OptimizeStatus::Canceled => {
                    ui.label(path.file_name());
                    ui.label(format!("({}KB)", size));
                }
            }
        });

        // 最終行以外は行下端に区切り線
        if index + 1 < total {
            ui.painter().hline(
                row_rect.x_range(),
                row_rect.bottom() + row_spacing * 0.5,
                ui.visuals().widgets.noninteractive.bg_stroke,
            );
        }

        // ホバーアクションを処理予約
        if response.hovered() {
            pending_action.push(FileListAction::Hover { id: *path.id() });
        }

        // クリックアクションを処理予約
        if response.clicked() {
            pending_action.push(FileListAction::Click { id: *path.id() });
            row_clicked = true;
        }

        // ダブルクリックアクションを処理予約
        if response.double_clicked() {
            pending_action.push(FileListAction::DoubleClick {
                path: path.path().clone(),
            });
        }
    }

    // クリックアクションを処理
    for action in pending_action {
        match action {
            FileListAction::Hover { id: _id } => {
                // ホバー
            }
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
            }
        }
    }

    row_clicked
}
