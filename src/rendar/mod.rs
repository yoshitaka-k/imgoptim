mod layout;
mod assets;
mod setting;

use crate::app;
use crate::file::open_files;
use crate::rendar::assets::{fonts, svg};
use crate::rendar::layout::{top, list, bottom};
use crate::rendar::setting as setting_window;
use crate::event::{open, drop, optimize};

const BOTTOM_BUTTON_HEIGHT: f32 = 30.0;

const DARK_MODE_BUTTON_COLOR: egui::Color32 = egui::Color32::from_rgb(200, 200, 200);
const LIGHT_MODE_BUTTON_COLOR: egui::Color32 = egui::Color32::from_rgb(130, 130, 130);


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
        self.optimize_job.result(&mut self.files, &mut self.is_optimizing);

        // 前フレームで予約された最適化を実行
        self.optimize();

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;
            // style.visuals.override_text_color = Some(egui::Color32::WHITE);

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
            top::top_layout(ui, &mut self.files, &mut self.open_dialog, &mut self.settings_window_open, &mut self.settings_window_pos);

            let row_height = list::row_height(ui);
            let total_rows = self.files.paths().len();

            // ファイル一覧を表示
            // リスト行をクリックしたら選択状態を保持
            let row_clicked = egui::ScrollArea::vertical()
                // コンテナが小さい時に縮小させない
                .auto_shrink([false; 2])
                // スクロールビューの高さを指定
                .max_height(ui.available_height() - BOTTOM_BUTTON_HEIGHT)
                // コンテナ内の表示
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    list::file_list(ui, &mut self.files, row_range, row_height)
                }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.files.set_selected_id(None);
            }

            // 下部ボタンを表示
            bottom::bottom_layout(ui, &mut self.files, &mut self.optimize_job);
        });

        // 設定ウィンドウを表示
        if self.settings_window_open {
            setting_window::setting_window(ui.ctx(), &mut self.app, &mut self.settings_window_open, &mut self.settings_window_pos);
        }
    }
}
