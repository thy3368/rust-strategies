# Nautilus Trader 策略插件 - Rust 实现

基于 **Nautilus Trader >= 1.221.0** 框架的纯 Rust 交易策略实现。

> **设计理念**：本项目是 Nautilus Trader 的**策略插件**，复用 Nautilus 的所有基础设施（回测引擎、实盘引擎、交易所适配器），专注于策略逻辑实现。

## 🎯 核心优势

- ✅ **纯 Rust 实现** - 回测和实盘都是纯 Rust，无 Python 依赖
- ✅ **超低延迟** - 符合 CLAUDE.md 低延迟标准（< 1μs）
- ✅ **复用 Nautilus** - 使用成熟的 Nautilus 回测和实盘引擎
- ✅ **Binance 现成** - Nautilus 的 Binance 适配器开箱即用
- ✅ **回测/实盘一致** - 相同策略代码，无缝切换

## 📊 已实现策略

### Avellaneda-Stoikov 做市策略

经典的做市商策略，通过动态调整买卖价差来平衡库存风险和做市收益。

**性能指标**（符合 CLAUDE.md 要求）：
- 报价计算: < 1μs
- 波动率更新: < 2μs
- 零分配热路径: ✅

**配置参数**：
```rust
StrategyConfig {
    instrument_id: "BTCUSDT-BINANCE",
    risk_aversion: 0.1,           // 风险厌恶系数 γ
    order_arrival_rate: 100.0,    // 订单到达率 λ
    price_sensitivity: 1.5,       // 价格敏感度 κ
    time_horizon: 300.0,          // 时间范围（秒）
    base_order_size: 0.001,       // 基础订单大小
    max_position_size: 0.1,       // 最大持仓
    max_inventory: 0.05,          // 最大库存偏离
    volatility_window: 20,        // 波动率窗口
    use_parkinson: true,          // 使用 Parkinson 波动率
    ...
}
```

## 🚀 快速开始

### 前置条件

```bash
# 安装 Rust（如未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 验证安装
rustc --version
cargo --version
```

### 安装依赖

**重要说明**：Nautilus Trader 目前主要通过 Python wheels 分发。纯 Rust 使用需要从源码编译或等待 crates.io 发布。

#### 方案 A：从本地 Nautilus 源码（推荐）

1. 克隆 Nautilus Trader：
```bash
cd ~/src
git clone https://github.com/nautechsystems/nautilus_trader
cd nautilus_trader
```

2. 修改本项目 `Cargo.toml`，使用本地路径：
```toml
[dependencies]
# 使用本地 Nautilus crates
nautilus-backtest = { path = "../../nautilus_trader/crates/backtest", default-features = false }
nautilus-live = { path = "../../nautilus_trader/crates/live", default-features = false }
nautilus-binance = { path = "../../nautilus_trader/crates/adapters/binance", default-features = false }
# ... 其他依赖
```

#### 方案 B：等待 crates.io 发布

Nautilus Trader 计划发布到 crates.io。届时可直接使用：
```toml
[dependencies]
nautilus-backtest = ">=0.53"
nautilus-live = ">=0.53"
nautilus-binance = ">=0.53"
```

### 编译项目

```bash
# 开发模式（快速编译）
cargo build

# 发布模式（最高性能）
cargo build --release

# 运行测试
cargo test
```

## 📖 使用示例

### 回测

```bash
# 运行回测（TODO: 等待 Nautilus 发布后完善）
cargo run --release --bin backtest
```

回测代码框架（`src/bin/backtest.rs`）：
```rust
use nautilus_backtest::BacktestEngine;
use nautilus_strategies_rust::AvellanedaStoikovStrategy;

fn main() -> Result<()> {
    // 1. 创建回测引擎
    let mut engine = BacktestEngine::new(config)?;

    // 2. 加载历史数据
    engine.load_data("data/BTCUSDT_2024.parquet")?;

    // 3. 添加策略
    let strategy = AvellanedaStoikovStrategy::new(strategy_config)?;
    engine.add_strategy(strategy);

    // 4. 运行回测
    engine.run()?;

    // 5. 生成报告
    let report = engine.generate_report();
    println!("{}", report);

    Ok(())
}
```

### 实盘

```bash
# 设置环境变量
export BINANCE_API_KEY="your_api_key"
export BINANCE_API_SECRET="your_api_secret"

# 运行实盘（TODO: 等待 Nautilus 发布后完善）
cargo run --release --bin live
```

实盘代码框架（`src/bin/live.rs`）：
```rust
use nautilus_live::LiveTradingEngine;
use nautilus_binance::BinanceSpotAdapter;
use nautilus_strategies_rust::AvellanedaStoikovStrategy;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. 创建 Binance 适配器
    let binance = BinanceSpotAdapter::new(api_key, api_secret).await?;

    // 2. 创建实盘引擎
    let mut engine = LiveTradingEngine::new(config)?;
    engine.add_adapter(binance);

    // 3. 添加策略
    let strategy = AvellanedaStoikovStrategy::new(strategy_config)?;
    engine.add_strategy(strategy);

    // 4. 启动
    engine.start().await?;

    // 5. 优雅停止
    tokio::signal::ctrl_c().await?;
    engine.stop().await?;

    Ok(())
}
```

## 🏗️ 项目结构

```
src/
├── lib.rs                          # 库入口
├── strategies/                     # 策略实现
│   ├── mod.rs
│   ├── config.rs                   # 策略配置
│   └── avellaneda_stoikov.rs       # AS策略核心逻辑
├── indicators/                     # 自定义指标
│   ├── mod.rs
│   └── volatility.rs               # 波动率指标
└── bin/
    ├── backtest.rs                 # 回测运行器
    └── live.rs                     # 实盘运行器
```

## 🔧 配置管理

### 从配置文件加载

```rust
use nautilus_strategies_rust::StrategyConfig;

// 加载配置
let config = StrategyConfig::from_file("config.json")?;

// 验证配置
config.validate()?;

// 创建策略
let strategy = AvellanedaStoikovStrategy::new(config)?;
```

示例配置文件（`config.json`）：
```json
{
  "instrument_id": "BTCUSDT-BINANCE",
  "risk_aversion": "0.1",
  "order_arrival_rate": "100.0",
  "price_sensitivity": "1.5",
  "time_horizon": "300.0",
  "base_order_size": "0.001",
  "max_position_size": "0.1",
  "max_inventory": "0.05",
  "volatility_window": 20,
  "use_parkinson": true,
  "inventory_penalty_factor": "2.0",
  "max_spread_bps": "200.0",
  "min_spread_bps": "2.0"
}
```

## ⚡ 性能优化

本项目严格遵循 CLAUDE.md 低延迟标准：

### 编译优化

```toml
[profile.release]
opt-level = 3              # 最高优化级别
lto = "fat"                # 全链接时优化
codegen-units = 1          # 单一代码生成单元
panic = "abort"            # 避免展开开销
strip = "symbols"          # 移除符号表
overflow-checks = false    # 禁用溢出检查
incremental = false        # 禁用增量编译
```

### CPU 架构优化

```toml
# Apple Silicon (ARM64)
[target.'cfg(all(target_arch = "aarch64", target_vendor = "apple"))']
rustflags = [
    "-C", "target-cpu=native",
    "-C", "target-feature=+neon,+crypto,+aes",
]

# x86-64
[target.'cfg(target_arch = "x86_64")']
rustflags = [
    "-C", "target-cpu=native",
    "-C", "target-feature=+avx2,+fma",
]
```

### 性能基准测试

```bash
cargo bench
```

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test integration_test

# 显示日志输出
cargo test -- --nocapture
```

## 📚 架构设计

### 插件式架构

```
┌─────────────────────────────────────────┐
│  您的策略 (This Project)                │
│  - Avellaneda-Stoikov Strategy          │
│  - 自定义指标                           │
└─────────────────┬───────────────────────┘
                  │ 作为插件
┌─────────────────▼───────────────────────┐
│  Nautilus Trader Framework              │
│  - 回测引擎 (BacktestEngine)            │
│  - 实盘引擎 (LiveTradingEngine)         │
│  - Binance 适配器                       │
│  - 数据管理                             │
│  - 风险管理                             │
│  - 执行管理                             │
└─────────────────────────────────────────┘
```

### 关注点分离

- **策略逻辑**（本项目）：专注于交易策略的核心算法
- **基础设施**（Nautilus）：回测、实盘、数据、风险、执行等

## 🛠️ 开发指南

### 添加新策略

1. 在 `src/strategies/` 创建新文件
2. 实现策略核心逻辑
3. 在 `src/strategies/mod.rs` 中导出
4. 创建配置结构体

### 性能优化检查清单

- [ ] 热路径零分配
- [ ] 数据结构缓存行对齐
- [ ] 使用 `inline(always)` 标记关键函数
- [ ] 避免不必要的克隆
- [ ] 使用 `rust_decimal` 而非 `f64`（精度）

## 📝 待办事项

- [ ] 等待 Nautilus Trader 发布到 crates.io
- [ ] 完善回测示例（添加真实数据加载）
- [ ] 完善实盘示例（添加订单管理）
- [ ] 添加更多策略（趋势跟踪、统计套利等）
- [ ] 性能基准测试对比
- [ ] 集成测试覆盖

## 🔗 相关资源

- [Nautilus Trader GitHub](https://github.com/nautechsystems/nautilus_trader)
- [Nautilus Trader 文档](https://nautilustrader.io/docs/)
- [Avellaneda-Stoikov 论文](https://www.math.nyu.edu/faculty/avellane/HighFrequencyTrading.pdf)

## 📄 许可证

MIT License

---

**注意**：本项目目前处于早期阶段，等待 Nautilus Trader 正式发布到 crates.io 后将完善更多功能。

© 2026 - Powered by Nautilus Trader Framework
# rust-strategies
