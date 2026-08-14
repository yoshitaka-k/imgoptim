use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use oxipng::{optimize_from_memory, Options};
use std::collections::HashSet;
use std::sync::Mutex;

/// PNG 最適化を行う構造体
pub struct Png;

impl Png {
    /// PNG ファイルを最適化
    /// * `path` - 最適化する PNG のパス
    /// * `options` - PNG 最適化オプション
    /// * `running` - 最適化中かどうか
    /// * `return` - 最適化の結果
    pub fn optimize(id: u64, path: &PathBuf, options: Options, running: Arc<AtomicBool>, canceled: Arc<Mutex<HashSet<u64>>>) -> Result<(), Box<dyn std::error::Error>> {
        // 先にファイルを読み込んでおく
        let input = std::fs::read(path)?;

        // 最適化中止された場合は処理を中断
        if !running.load(Ordering::Relaxed) || canceled.lock().unwrap().contains(&id) {
            return Ok(());
        }

        // oxipng でロスレス最適化（パレット維持・ビット深度削減・再圧縮）
        let output = optimize_from_memory(&input, &options)?;

        // 最適化中止された場合は処理を中断
        if !running.load(Ordering::Relaxed) || canceled.lock().unwrap().contains(&id) {
            return Ok(());
        }

        // ファイルを上書き保存
        std::fs::write(path, output)?;

        Ok(())
    }
}
