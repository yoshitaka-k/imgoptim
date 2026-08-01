mod fonts;

use crate::app;
use crate::drop_file;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    drop_file: drop_file::DropFile,
    is_optimizing: bool,
}

impl Rendar {
    /// 新しい Rendar を作成
    /// * `cc` - 作成コンテキスト
    /// * `app` - アプリケーション
    /// * `return` - Rendar のインスタンス
    pub fn new(cc: &eframe::CreationContext, app: app::App) -> Self {
        // フォントを追加
        fonts::install(&cc.egui_ctx);

        let drop_file = drop_file::DropFile::new();
        Self { app, drop_file, is_optimizing: false }
    }

    /// ファイルを最適化
    fn optimize(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.drop_file.optimize(&self.app) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl eframe::App for Rendar {
    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_optimizing {
            self.is_optimizing = false;

            match self.optimize() {
                Ok(_) => {},
                Err(e) => eprintln!("optimize failed: {}", e),
            }
        }

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // ドラッグ&ドロップされたファイルを追加
        ui.ctx().input(|input| {
            self.drop_file.set_extensions(self.app.extensions().clone());

            let files = input.raw.dropped_files.clone();
            if files.len() > 0 {
                for file in files {
                    if let Some(path) = file.path {
                        self.drop_file.add_path(path);
                    }
                }

                self.is_optimizing = true;
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let availabel_width = ui.available_size().x;

            ui.label("File Drag & Drop");

            ui.separator();

            // ファイル一覧を表示
            egui::ScrollArea::both()
                .max_width(availabel_width)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (index, path) in self.drop_file.paths().iter().enumerate() {
                        match path.path().metadata() {
                            Ok(metadata) => {
                                let size = metadata.len() / 1024;
                                ui.label(format!(
                                    "{} ({} KB)",
                                    path.path().file_name().unwrap().to_str().unwrap(),
                                    size
                                ));

                                if index < self.drop_file.paths().len() - 1 {
                                    ui.separator();
                                }
                            }
                            Err(_) => {}
                        }
                    }
                });

            // 左寄せ
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if ui.button("Open").clicked() {
                    let extensions: Vec<String> = self.app.extensions()
                        .iter()
                        .map(|ext| ext.to_string())
                        .collect();

                    // Macのみファイルとフォルダを同時選択できる
                    #[cfg(target_os = "macos")]
                    let paths = rfd::FileDialog::new()
                        .add_filter("Images", &extensions)
                        .pick_files_or_folders();

                    // Mac以外は複数フォルダ選択のみ
                    #[cfg(not(target_os = "macos"))]
                    let paths = rfd::FileDialog::new()
                        .add_filter("Images", &extensions)
                        .pick_folders();

                    // ファイルを追加
                    if let Some(paths) = paths {
                        for path in paths {
                            self.drop_file.add_path(path);
                        }

                        self.is_optimizing = true;
                    }
                }
            });

            // 右寄せ
            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Clear").clicked() {
                        self.drop_file.clear();
                    }
                });
            });
        });
    }
}
