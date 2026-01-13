//! Nautilus Trader 实盘交易程序
//!
//! 基于 Nautilus Trader 官方实盘引擎的高性能交易系统
//! 实现 Avellaneda-Stoikov 做市策略的实盘交易
//!
//! 用法:
//! ```bash
//! cargo run --release --bin live
//! ```

use anyhow::Result;
use dotenv::dotenv;
use nautilus_binance::config::{BinanceDataClientConfig, BinanceExecClientConfig};
use nautilus_binance::factories::{BinanceDataClientFactory, BinanceExecutionClientFactory};
use nautilus_binance::common::enums::{BinanceEnvironment, BinanceProductType};
use nautilus_common::enums::Environment;
use nautilus_live::builder::LiveNodeBuilder;
use nautilus_model::identifiers::{AccountId, TraderId};
use tracing::{info, warn};

use nautilus_strategies_rust::strategies::nautilus_compatible::create_strategy;

fn main() -> Result<()> {
    // 加载环境变量 - 明确指定 .env 文件路径
    let env_path = std::env::current_dir()
        .map(|dir| dir.join(".env"))
        .map_err(|e| anyhow::anyhow!("无法获取当前目录: {}", e))?;

    if env_path.exists() {
        info!("加载环境变量文件: {:?}", env_path);
        dotenv::from_path(&env_path)?;
    } else {
        warn!("未找到 .env 文件: {:?}", env_path);
    }

    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("=================================================================");
    info!("Nautilus Trader - 实盘交易引擎");
    info!("版本: {}", env!("CARGO_PKG_VERSION"));
    info!("=================================================================");

    // 从环境变量加载 Binance API 凭证
    let api_key = std::env::var("BINANCE_API_KEY").ok();
    let api_secret = std::env::var("BINANCE_API_SECRET").ok();
    let is_testnet = std::env::var("BINANCE_TESTNET")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    if api_key.is_none() || api_secret.is_none() {
        warn!("未检测到 Binance API 凭证环境变量");
        warn!("请在 .env 文件中设置:");
        warn!("  BINANCE_API_KEY=\"your_api_key\"");
        warn!("  BINANCE_API_SECRET=\"your_api_secret\"");
        return Ok(());
    }

    info!("使用 Binance {} 环境", if is_testnet { "测试网" } else { "实盘" });

    // 运行实盘交易
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run_live(api_key.unwrap(), api_secret.unwrap(), is_testnet))
}

/// 运行实盘交易
async fn run_live(api_key: String, api_secret: String, is_testnet: bool) -> Result<()> {
    info!("配置实盘交易引擎...");

    // 创建实盘节点构建器
    let mut builder = LiveNodeBuilder::new(
        TraderId::from("TRADER-001"),
        if is_testnet { Environment::Sandbox } else { Environment::Live },
    )?;

    // 配置节点参数
    builder = builder
        .with_name("Avellaneda-Stoikov-Market-Maker")
        .with_timeout_connection(60)
        .with_timeout_reconciliation(30)
        .with_timeout_portfolio(10)
        .with_timeout_disconnection_secs(10)
        .with_delay_post_stop_secs(10)
        .with_delay_shutdown_secs(5)
        .with_reconciliation(true)
        .with_reconciliation_lookback_mins(60);

    info!("配置 Binance 数据客户端...");
    let data_config = BinanceDataClientConfig {
        product_types: vec![BinanceProductType::Spot],
        environment: if is_testnet { BinanceEnvironment::Testnet } else { BinanceEnvironment::Mainnet },
        api_key: Some(api_key.clone()),
        api_secret: Some(api_secret.clone()),
        ..Default::default()
    };

    builder = builder.add_data_client(
        None,
        Box::new(BinanceDataClientFactory::new()),
        Box::new(data_config),
    )?;

    info!("配置 Binance 执行客户端...");
    let exec_config = BinanceExecClientConfig {
        trader_id: TraderId::from("TRADER-001"),
        account_id: AccountId::from("BINANCE-001"),
        product_types: vec![BinanceProductType::Spot],
        environment: if is_testnet { BinanceEnvironment::Testnet } else { BinanceEnvironment::Mainnet },
        api_key: Some(api_key),
        api_secret: Some(api_secret),
        ..Default::default()
    };

    builder = builder.add_exec_client(
        None,
        Box::new(BinanceExecutionClientFactory::new()),
        Box::new(exec_config),
    )?;

    info!("创建实盘节点实例...");
    let mut node = builder.build()?;
    info!("✅ 实盘节点创建成功");

    info!("添加 Avellaneda-Stoikov 策略...");
    let strategy = create_strategy();
    node.add_strategy(strategy)?;
    info!("✅ 策略添加完成");

    info!("🚀 启动实盘交易...");
    node.run().await?;

    info!("");
    info!("=================================================================");
    info!("✅ 实盘交易系统执行完成");
    info!("=================================================================");

    Ok(())
}
