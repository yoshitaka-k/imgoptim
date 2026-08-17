mod layout;
mod assets;
mod setting;

use crate::app;
use crate::file::open_files;
use crate::rendar::assets::{fonts, svg};
use crate::rendar::layout::{top, list, bottom};
use crate::rendar::setting as setting_window;
use crate::event::{open, drop, optimize};

/// レンダーを管理する構造体
pub struct Rendar {
    app: app::App,
    files: open_files::OpenFiles,

    // ファイルダイアログを開くタイミング
    open_dialog: bool,

    // 設定ウィンドウを開くタイミング
    settings_window_open: bool,

    // 設定ウィンドウの表示位置
    settings_window_pos: Option<egui::Pos2>,

    // 最適化ジョブ
    optimize_job: optimize::OptimizeJob,
}

impl Rendar {
    /// 新しい Rendar を作成
    /// * `cc` - 作成コンテキスト
    /// * `app` - アプリケーション
    /// * `return` - Rendar のインスタンス
    pub fn new(cc: &eframe::CreationContext<'_>, app: app::App) -> Self {
        // フォントと SVG ローダーを追加
        fonts::install(&cc.egui_ctx);
        svg::install(&cc.egui_ctx);

        // 前回保存した App があれば復元（なければ引数の app を使う）
        let app = cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or(app);

        // 開くファイルのインスタンスを作成
        let mut files = open_files::OpenFiles::new();
        // 拡張子を設定
        files.set_extensions(app.extensions_to_string());

        Self {
            app,
            files,
            open_dialog: false,
            settings_window_open: false,
            settings_window_pos: None,
            optimize_job: optimize::OptimizeJob::new(cc.egui_ctx.clone()),
        }
    }

    /// ファイルを最適化
    fn optimize(&mut self) {
        // 未処理がなければ何もしない
        if !self.files.has_standby() {
            return;
        }

        // 前の最適化のスレッドが生きていれば待つ
        if !self.optimize_job.is_running_count_zero() {
            return;
        }

        // 最適化を実行するスレッドの準備
        self.optimize_job.run(&self.app, &mut self.files);
    }
}

impl eframe::App for Rendar {
    /// 終了前に App の状態を保存
    /// * `storage` - ストレージ
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.app);
    }

    /// ユーザーインターフェースを描画
    /// * `ui` - ユーザーインターフェース
    /// * `frame` - フレーム
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 最適化結果を反映
        self.optimize_job.result(&mut self.files);

        // 結果を反映後、まだ未処理があれば最適化を実行
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
            );
        }

        // ドラッグ&ドロップされたファイルを処理
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            drop::drop_files(
                &files,
                &mut self.files,
            );
        });

        // ファイル追加時に最適化を実行
        self.optimize();

        // パネルの背景色を取得
        let panel_fill_color = assets::panel_fill_color(ui);

        // パネルのスタイルを設定
        let panel_style = egui::Frame::default()
            .fill(panel_fill_color)
            .inner_margin(egui::Margin {
                left: 10,
                right: 10,
                top: 2,
                bottom: 3,
            });

        // 上部ボタンを表示
        egui::Panel::top("top_taskbar").frame(panel_style).show(ui, |ui| {
            top::top_layout(ui, &mut self.files, &mut self.open_dialog, &mut self.settings_window_open, &mut self.settings_window_pos);
        });

        // 状態とかボタンを表示するタスクバーを表示
        egui::Panel::bottom("bottom_taskbar").frame(panel_style).show(ui, |ui| {
            bottom::bottom_layout(ui, &mut self.files, &mut self.optimize_job);
        });

        // 中央パネルを表示
        egui::CentralPanel::default().show(ui, |ui| {
            let row_height = list::row_height(ui);
            let total_rows = self.files.paths().len();

            // ファイル一覧を表示
            // リスト行をクリックしたら選択状態を保持
            let row_clicked = egui::ScrollArea::vertical()
                // コンテナが小さい時に縮小させない
                .auto_shrink([false; 2])
                // スクロールビューの高さを指定
                .max_height(ui.available_height())
                // コンテナ内の表示
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    // ファイル一覧を表示
                    list::file_list(ui, &mut self.files, &mut self.optimize_job, row_range, row_height)
                }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.files.set_selected_id(None);
            }
        });

        // 設定ウィンドウを表示
        if self.settings_window_open {
            setting_window::setting_window(ui.ctx(), &mut self.app, &mut self.settings_window_open, &mut self.settings_window_pos);
        }
    }
}
