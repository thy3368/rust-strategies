//! 策略实例化和配置验证测试
//!
//! 用于测试 Avellaneda-Stoikov 策略与 Nautilus Trader 接口的兼容性

use nautilus_strategies_rust::strategies::nautilus_compatible::create_strategy;

fn main() {
    println!("开始策略实例化测试...");

    // 1. 测试策略创建（不返回 Result）
    let strategy = create_strategy();
    println!("✅ 策略创建成功");

    // 2. 打印策略状态
    println!("初始状态: {:?}", strategy.state());

    println!("✅ 策略实例化测试通过!");
}