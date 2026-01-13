//! Trading Strategies - 交易策略模块
//!
//! 作为 Nautilus Trader 的插件实现各种交易策略

pub mod avellaneda_stoikov;
pub mod nautilus_compatible;

pub use avellaneda_stoikov::{AvellanedaStoikov, ASConfig};
pub use nautilus_compatible::{
    NautilusAvellanedaStoikov,
    NautilusASConfig,
    create_strategy,
    create_strategy_with_config,
};
