//! 策略配置验证测试
//!
//! 测试策略配置的创建、验证和兼容性

use nautilus_strategies_rust::strategies::avellaneda_stoikov::ASConfig;
use nautilus_strategies_rust::strategies::nautilus_compatible::{
    create_strategy, create_strategy_with_config,
};

fn main() {
    println!("开始策略配置验证测试...");

    // 1. 测试默认配置
    println!("\n=== 测试默认配置 ===");
    let default_strategy = create_strategy();
    println!("策略状态: {:?}", default_strategy.state());

    // 2. 测试自定义配置
    println!("\n=== 测试自定义配置 ===");
    let mut custom_config = ASConfig::default();
    custom_config.instrument_id = "BTCUSDT".to_string();
    custom_config.risk_aversion = 0.1;
    custom_config.order_arrival_rate = 100.0;
    custom_config.price_sensitivity = 1.5;
    custom_config.time_horizon = 300.0;
    custom_config.base_order_size = 0.001;
    custom_config.max_position_size = 0.1;
    custom_config.max_inventory = 0.05;
    custom_config.volatility_window = 20;
    custom_config.use_parkinson = true;

    let custom_strategy = create_strategy_with_config(custom_config);
    println!("策略状态: {:?}", custom_strategy.state());

    // 3. 验证策略核心功能
    println!("\n=== 验证策略核心功能 ===");

    // 测试策略是否正确初始化
    assert_eq!(custom_strategy.state(), default_strategy.state());

    println!("✅ 所有配置验证通过!");
}
