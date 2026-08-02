mod layout;
mod fonts;
mod event;

use crate::app;
use crate::file::open_files;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    files: open_files::OpenFiles,

    open_dialog: bool,
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

        let mut files = open_files::OpenFiles::new();
        files.set_extensions(app.extensions_to_string());

        Self {
            app,
            files,
            open_dialog: false,
            is_optimizing: false,
        }
    }

    /// ファイルを最適化
    fn optimize(&mut self) {
        if !self.is_optimizing {
            return;
        }

        self.is_optimizing = false;

        if let Err(e) = self.files.optimize(&self.app) {
            eprintln!("optimize failed: {}", e);
        }
    }
}

impl eframe::App for Rendar {
    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 前フレームで予約された最適化を実行
        self.optimize();

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // ファイルダイアログをリスト描画前に開く
        if self.open_dialog {
            self.open_dialog = false;
            event::open_button_clicked(
                &self.app.extensions_to_string(),
                &mut self.files,
                &mut self.is_optimizing,
            );
        }

        // ドラッグ&ドロップされたファイルを処理
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            event::drop_files(&files, &mut self.files, &mut self.is_optimizing);
        });

        // 中央パネルを表示
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let availabel_width = ui.available_size().x;

            ui.label("File Drag & Drop");

            ui.separator();

            // ファイル一覧を表示
            egui::ScrollArea::both()
                .max_width(availabel_width)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    layout::list::file_list(ui, &self.files);
                });

            // 下部ボタンを表示（見た目の位置はそのまま）
            layout::button::bottom_button(ui, &mut self.files, &mut self.open_dialog);
        });
    }
}
