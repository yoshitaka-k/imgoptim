/// 最適化ステータス
#[derive(Clone, PartialEq)]
pub enum OptimizeStatus {
    /// 最適化未実行
    None,
    /// 最適化中
    Optimizing,
    /// 最適化完了
    Optimized,
    /// 最適化エラー（メッセージ）
    Error(String),
}
