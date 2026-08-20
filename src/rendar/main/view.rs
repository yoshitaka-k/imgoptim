use crate::app;
use crate::file::open_files;
use crate::event::{open, drop};
use crate::optimize::OptimizeJob;
use crate::rendar;
use crate::rendar::SettingTab;
use crate::rendar::assets::{fonts, svg};
use crate::rendar::main::{top, list, bottom, modal};
use crate::rendar::setting::view as setting_window;

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

    // 設定タブ
    setting_tab: SettingTab,

    // 最適化ジョブ
    optimize_job: OptimizeJob,

    // エラーモーダルを表示するかどうか
    error_modal_open: bool,

    // エラー
    error: Option<Box<dyn std::error::Error>>,
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
            setting_tab: SettingTab::Concurrent,
            optimize_job: OptimizeJob::new(cc.egui_ctx.clone()),
            error_modal_open: false,
            error: None,
        }
    }

    /// ファイルを最適化実行
    fn optimize_run(&mut self) {
        self.optimize_job.run(&self.app, &mut self.files);
    }

    /// 最適化結果を反映する
    fn optimize_result(&mut self) {
        self.optimize_job.result(&mut self.files);
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
        self.optimize_result();

        // 結果を反映後、まだ未処理があれば最適化を実行
        self.optimize_run();

        // スタイルを設定
        ui.ctx().global_style_mut(|style| {
            // ラベルを選択できないようにする
            style.interaction.selectable_labels = false;

        });

        // 開くボタンが押されてたらファイルダイアログを開く
        if self.open_dialog {
            self.open_dialog = false;
            if let Err(e) = open::open_files(
                &self.app.extensions_to_string(),
                &mut self.files,
            ) {
                eprintln!("Error opening files: {}", e);
                self.error = Some(e);
                self.error_modal_open = true;
            }
        }

        // ドラッグ&ドロップされたファイルを処理
        ui.ctx().input(|input| {
            let files = input.raw.dropped_files.clone();
            if let Err(e) = drop::drop_files(
                &files,
                &mut self.files,
            ) {
                eprintln!("Error opening files: {}", e);
                self.error = Some(e);
                self.error_modal_open = true;
            }
        });

        // ファイル追加時に最適化を実行
        self.optimize_run();

        // パネルのスタイルを設定
        let panel_style = rendar::panel_style(ui);

        // 上部ボタンを表示
        egui::Panel::top("top_taskbar").frame(panel_style).show(ui, |ui| {
            top::view(ui, &mut self.files, &mut self.open_dialog, &mut self.settings_window_open, &mut self.settings_window_pos);
        });

        // 状態とかボタンを表示するタスクバーを表示
        egui::Panel::bottom("bottom_taskbar").frame(panel_style).show(ui, |ui| {
            bottom::view(ui, &mut self.files, &mut self.optimize_job, &mut self.error_modal_open, &mut self.error);
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
                    list::view(ui, &mut self.files, &mut self.optimize_job, row_range, row_height, &mut self.error_modal_open, &mut self.error)
                }).inner;

            // リスト行以外をクリックしたら選択解除
            if ui.input(|i| i.pointer.primary_clicked()) && !row_clicked {
                self.files.set_selected_id(None);
            }
        });

        // 設定ウィンドウを表示
        if self.settings_window_open {
            setting_window::view(ui.ctx(), &mut self.app, &mut self.setting_tab, &mut self.settings_window_open, &mut self.settings_window_pos);
        }

        // エラーモーダルを表示
        if self.error_modal_open {
            modal::error(&mut self.error_modal_open, ui.ctx(), &mut self.error);
        }
    }
}
