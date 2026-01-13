//! Nautilus Trader 策略插件
//!
//! 基于 Nautilus Trader 框架的高性能交易策略实现。
//! 复用 Nautilus 的回测引擎、实盘引擎和交易所适配器。

pub mod strategies;
pub mod indicators;

// 导出主要组件
pub use strategies::avellaneda_stoikov::{AvellanedaStoikov, ASConfig};


/// 库版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 缓存行大小（Apple M系列为128字节）
#[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
pub const CACHE_LINE_SIZE: usize = 128;

#[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
pub const CACHE_LINE_SIZE: usize = 64;

/// 缓存行对齐封装
#[repr(align(128))]
#[derive(Debug, Clone, Copy)]
pub struct CacheAligned<T> {
    pub data: T,
}

impl<T> CacheAligned<T> {
    #[inline(always)]
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
