/// 最適化ステータス
#[derive(Clone, PartialEq)]
pub enum OptimizeStatus {
    /// 最適化未実行
    Standby,
    /// 最適化中
    Optimizing,
    /// 最適化完了
    Optimized,
    /// 最適化エラー（メッセージ）
    Error(String),
    /// 最適化キャンセル
    Canceled,
}
