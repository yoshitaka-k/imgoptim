mod layout;
mod assets;

use crate::app;
use crate::file::open_files;
use crate::rendar::assets::{fonts, svg};
use crate::rendar::layout::{top, list, bottom};
use crate::event::{open, drop, optimize};

const BOTTOM_BUTTON_HEIGHT: f32 = 30.0;

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    files: open_files::OpenFiles,

    // ファイルダイアログを開くタイミング
    open_dialog: bool,

    // 最適化中かどうか
    is_optimizing: bool,

    // 最適化ジョブ
    optimize_job: optimize::OptimizeJob,
}

impl Rendar {
    /// 新しい Rendar を作成
    /// * `cc` - 作成コンテキスト
    /// * `app` - アプリケーション
    /// * `return` - Rendar のインスタンス
    pub fn new(cc: &eframe::CreationContext, app: app::App) -> Self {
        // フォントと SVG ローダーを追加
        fonts::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        let mut files = open_files::OpenFiles::new();
        files.set_extensions(app.extensions_to_string());

        Self {
            app,
            files,
            open_dialog: false,
            is_optimizing: false,
            optimize_job: optimize::OptimizeJob::new(cc.egui_ctx.clone()),
        }
    }

    /// ファイルを最適化
    fn optimize(&mut self) {
        if !self.is_optimizing {
            return;
        }

        // すでに別スレッドで最適化中なら、完了後に再開する
        if self.files.has_optimizing() {
            return;
        }

        // 未処理がなければ何もしない
        if !self.files.has_pending() {
            self.is_optimizing = false;
            return;
        }

        self.is_optimizing = false;

        // 最適化を実行するスレッドの準備
        self.optimize_job.run(&self.app, &mut self.files);
    }
}

impl eframe::App for Rendar {
    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 最適化結果を反映
        self.optimize_job.result(&mut self.files, &mut self.is_optimizing);

        // 前フレームで予約された最適化を実行
        self.optimize();

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
        });

        // 開くボタンが押されてたらファイルダイアログを開く
        if self.open_dialog {
            self.open_dialog = false;
            open::open_files(
                &self.app.extensions_to_string(),
                &mut self.files,
                &mut self.is_optimizing,
            );
        }

        // ドラッグ&ドロップされたファイルを処理
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            drop::drop_files(&files, &mut self.files, &mut self.is_optimizing);
        });

        // 中央パネルを表示
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // 上部ボタンを表示
            top::top_layout(ui, &mut self.files, &mut self.open_dialog);

            // ファイル一覧を表示
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .max_height(ui.available_height() - BOTTOM_BUTTON_HEIGHT)
                .show(ui, |ui| {
                    list::file_list(ui, &mut self.files);
                });

            // 下部ボタンを表示
            bottom::bottom_layout(ui, &mut self.files);
        });
    }
}
