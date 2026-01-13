//! Strategy Configuration - 策略配置

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Avellaneda-Stoikov 做市策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    /// 交易品种 ID（例如：BTCUSDT-BINANCE）
    pub instrument_id: String,

    /// 风险厌恶系数 γ (gamma)
    pub risk_aversion: Decimal,

    /// 订单到达率 λ (lambda)
    pub order_arrival_rate: Decimal,

    /// 价格敏感度 κ (kappa)
    pub price_sensitivity: Decimal,

    /// 时间范围（秒）
    pub time_horizon: Decimal,

    /// 基础订单大小
    pub base_order_size: Decimal,

    /// 最大持仓
    pub max_position_size: Decimal,

    /// 最大库存偏离
    pub max_inventory: Decimal,

    /// 波动率窗口
    pub volatility_window: usize,

    /// 使用 Parkinson 波动率
    pub use_parkinson: bool,

    /// 库存惩罚因子
    pub inventory_penalty_factor: Decimal,

    /// 最大价差（基点）
    pub max_spread_bps: Decimal,

    /// 最小价差（基点）
    pub min_spread_bps: Decimal,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        Self {
            instrument_id: "BTCUSDT-BINANCE".to_string(),
            risk_aversion: Decimal::new(1, 1), // 0.1
            order_arrival_rate: Decimal::new(100, 0), // 100.0
            price_sensitivity: Decimal::new(15, 1), // 1.5
            time_horizon: Decimal::new(300, 0), // 300.0 秒
            base_order_size: Decimal::new(1, 3), // 0.001 BTC
            max_position_size: Decimal::new(1, 1), // 0.1 BTC
            max_inventory: Decimal::new(5, 2), // 0.05 BTC
            volatility_window: 20,
            use_parkinson: true,
            inventory_penalty_factor: Decimal::new(2, 0), // 2.0
            max_spread_bps: Decimal::new(200, 0), // 200 bps = 2%
            min_spread_bps: Decimal::new(2, 0), // 2 bps = 0.02%
        }
    }
}

impl StrategyConfig {
    /// 从 JSON 文件加载配置
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// 保存配置到 JSON 文件
    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), String> {
        if self.risk_aversion <= Decimal::ZERO {
            return Err("风险厌恶系数必须大于 0".to_string());
        }

        if self.base_order_size <= Decimal::ZERO {
            return Err("基础订单大小必须大于 0".to_string());
        }

        if self.max_spread_bps <= self.min_spread_bps {
            return Err("最大价差必须大于最小价差".to_string());
        }

        if self.volatility_window == 0 {
            return Err("波动率窗口必须大于 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = StrategyConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_risk_aversion() {
        let mut config = StrategyConfig::default();
        config.risk_aversion = Decimal::ZERO;
        assert!(config.validate().is_err());
    }
}
