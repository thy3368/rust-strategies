//! Trading Strategies - 交易策略模块
//!
//! 作为 Nautilus Trader 的插件实现各种交易策略

pub mod config;
pub mod avellaneda_stoikov;

pub use config::StrategyConfig;
pub use avellaneda_stoikov::AvellanedaStoikov;
