mod layout;
mod fonts;

use crate::app;
use crate::file::open_files;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    file: open_files::OpenFiles,
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

        let file = open_files::OpenFiles::new();
        Self { app, file, is_optimizing: false }
    }

    /// ファイルを最適化
    fn optimize(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.file.optimize(&self.app) {
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
            self.file.set_extensions(self.app.extensions().clone());

            let files = input.raw.dropped_files.clone();
            if files.len() > 0 {
                for file in files {
                    if let Some(path) = file.path {
                        self.file.add_path(path);
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
                    layout::list::file_list(ui, &self.file);
                });

            // 下部ボタンを表示
            layout::button::bottom_button(ui, &self.app, &mut self.file, &mut self.is_optimizing);
        });
    }
}
