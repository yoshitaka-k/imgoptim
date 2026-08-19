use std::path::PathBuf;
use crate::optimize::{OptimToken, OptimizeStatus};

/// PNG 最適化を行う構造体
pub struct Png;

impl Png {
    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - PNG 最適化オプション
    /// * `token` - 最適化トークン
    /// * `return` - 最適化の結果
    pub fn optimize(path: &PathBuf, options: oxipng::Options, token: OptimToken) -> Result<OptimizeStatus, Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let input = std::fs::read(path)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
        let output = oxipng::optimize_from_memory(&input, &options)?;

        // 最適化中止された場合は処理を中断
        if token.is_canceled() {
            return Ok(OptimizeStatus::Canceled);
        }

        // ファイルを上書き保存
        std::fs::write(path, output)?;

        Ok(OptimizeStatus::Optimized)
    }
}
