//! Nautilus Trader 兼容的 Avellaneda-Stoikov 策略
//!
//! 本策略实现了 Nautilus Trader 的策略接口，支持使用官方回测引擎
//! 进行策略回测和实盘交易。

use crate::strategies::avellaneda_stoikov::{
    ASConfig, AvellanedaStoikov as BaseStrategy, OrderBookSnapshot, QuoteUpdate,
};
use anyhow::Result;
use nautilus_common::actor::{Component, DataActor, DataActorCore};
use nautilus_common::cache::Cache;
use nautilus_common::clock::Clock;
use nautilus_common::enums::{ComponentState, ComponentTrigger};
use nautilus_model::enums::{OrderSide, TimeInForce};
use nautilus_model::events::order::{
    canceled::OrderCanceled, filled::OrderFilled, rejected::OrderRejected,
};
use nautilus_model::identifiers::{ComponentId, InstrumentId, StrategyId, TraderId};
use nautilus_model::orderbook::OrderBook as NautilusOrderBook;
use nautilus_model::types::{Price, Quantity};
use nautilus_trading::strategy::{Strategy, StrategyConfig, StrategyCore};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

/// 与 Nautilus 兼容的策略配置
#[derive(Debug, Clone)]
pub struct NautilusASConfig {
    /// 基础策略配置
    pub base_config: ASConfig,
    /// Nautilus 策略配置
    pub strategy_config: StrategyConfig,
}

impl Default for NautilusASConfig {
    fn default() -> Self {
        Self {
            base_config: ASConfig::default(),
            strategy_config: StrategyConfig::default(),
        }
    }
}

impl NautilusASConfig {
    pub fn new(base_config: ASConfig) -> Self {
        Self {
            base_config,
            strategy_config: StrategyConfig {
                strategy_id: Some(StrategyId::from("AV-STO-001")),
                order_id_tag: Some("AVSTO".to_string()),
                ..Default::default()
            },
        }
    }
}

/// 与 Nautilus 兼容的 Avellaneda-Stoikov 策略
pub struct NautilusAvellanedaStoikov {
    /// 策略核心组件（包含 DataActorCore）
    core: StrategyCore,
    /// 基础策略实现
    base_strategy: BaseStrategy,
    /// 工具ID
    instrument_id: InstrumentId,
    /// 当前报价
    current_quote: Option<QuoteUpdate>,
    /// 是否正在交易
    is_trading: bool,
}

impl NautilusAvellanedaStoikov {
    pub fn new(config: NautilusASConfig) -> Self {
        let instrument_id = InstrumentId::from(config.base_config.instrument_id.as_str());
        Self {
            core: StrategyCore::new(config.strategy_config),
            base_strategy: BaseStrategy::new(config.base_config),
            instrument_id,
            current_quote: None,
            is_trading: false,
        }
    }

    /// 从基础策略配置创建
    pub fn from_base_config(base_config: ASConfig) -> Self {
        Self::new(NautilusASConfig::new(base_config))
    }

    /// 更新订单
    fn update_orders(&mut self) -> Result<()> {
        if let Some(quote) = self.current_quote {
            // 取消现有订单
            self.cancel_all_orders(self.instrument_id, None, None)?;

            // 提交新订单 - 使用 PRICE_PRECISION 和 QUANTITY_PRECISION（假设为 2 和 4，可根据实际调整）
            const PRICE_PRECISION: u8 = 2;
            const QUANTITY_PRECISION: u8 = 4;

            let bid_order = self.create_limit_order(
                self.instrument_id,
                OrderSide::Buy,
                Price::new(quote.bid_price, PRICE_PRECISION),
                Quantity::new(quote.bid_size, QUANTITY_PRECISION),
            )?;

            let ask_order = self.create_limit_order(
                self.instrument_id,
                OrderSide::Sell,
                Price::new(quote.ask_price, PRICE_PRECISION),
                Quantity::new(quote.ask_size, QUANTITY_PRECISION),
            )?;

            self.submit_order(bid_order, None, None)?;
            self.submit_order(ask_order, None, None)?;
        }

        Ok(())
    }

    /// 创建限价订单
    fn create_limit_order(
        &mut self,
        instrument_id: InstrumentId,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
    ) -> Result<nautilus_model::orders::OrderAny> {
        let core = self.core_mut();
        let order_factory = core
            .order_factory
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("OrderFactory not available"))?;

        Ok(order_factory.limit(
            instrument_id,
            side,
            quantity,
            price,
            Some(TimeInForce::Gtc), // 当日有效（使用 Some 包装）
            None,                   // 过期时间
            None,                   // 只做市
            None,                   // 只减仓
            None,                   // 报价数量
            None,                   // 显示数量
            None,                   // 模拟触发
            None,                   // 触发工具ID
            None,                   // 执行算法ID
            None,                   // 执行算法参数
            None,                   // 标签
            None,                   // 客户端订单ID
        ))
    }
}

impl Component for NautilusAvellanedaStoikov {
    fn component_id(&self) -> ComponentId {
        todo!()
    }

    fn state(&self) -> ComponentState {
        todo!()
    }

    fn transition_state(&mut self, _trigger: ComponentTrigger) -> Result<()> {
        todo!()
    }

    fn register(
        &mut self,
        _trader_id: TraderId,
        _clock: Rc<RefCell<dyn Clock>>,
        _cache: Rc<RefCell<Cache>>,
    ) -> Result<()> {
        todo!()
    }
}

// 实现策略接口
impl Strategy for NautilusAvellanedaStoikov {
    fn core_mut(&mut self) -> &mut StrategyCore {
        &mut self.core
    }

    // 策略启动时调用
    fn on_start(&mut self) -> Result<()> {
        log::info!("Avellaneda-Stoikov 策略启动");

        // 开始交易
        self.is_trading = true;

        Ok(())
    }

    // 订单拒绝时调用
    fn on_order_rejected(&mut self, event: OrderRejected) {
        log::warn!("订单拒绝: {} - {}", event.client_order_id, event.reason);
    }
}

// 实现 DataActor 接口
impl DataActor for NautilusAvellanedaStoikov {
    // 策略停止时调用
    fn on_stop(&mut self) -> Result<()> {
        log::info!("Avellaneda-Stoikov 策略停止");

        // 停止交易
        self.is_trading = false;

        // 取消所有订单
        self.cancel_all_orders(self.instrument_id, None, None)?;

        Ok(())
    }

    // 订单簿数据更新时调用
    fn on_book(&mut self, order_book: &NautilusOrderBook) -> Result<()> {
        if !self.is_trading {
            return Ok(());
        }

        // 转换为基础策略格式
        let snapshot = OrderBookSnapshot {
            best_bid: order_book.best_bid_price().map(|p| p.as_f64()).unwrap_or(0.0),
            best_ask: order_book.best_ask_price().map(|p| p.as_f64()).unwrap_or(0.0),
            bid_volume: order_book
                .best_bid_size()
                .map(|q| q.as_f64())
                .unwrap_or(0.0),
            ask_volume: order_book
                .best_ask_size()
                .map(|q| q.as_f64())
                .unwrap_or(0.0),
            timestamp_ns: order_book.ts_last,
        };

        // 更新策略状态
        self.current_quote = self.base_strategy.on_orderbook_update(&snapshot);

        // 更新订单
        self.update_orders()?;

        Ok(())
    }

    // 订单成交时调用
    fn on_order_filled(&mut self, event: &OrderFilled) -> Result<()> {
        // 更新库存
        let side = match event.order_side {
            OrderSide::Buy => crate::strategies::avellaneda_stoikov::OrderSide::Buy,
            OrderSide::Sell => crate::strategies::avellaneda_stoikov::OrderSide::Sell,
            OrderSide::NoOrderSide => return Ok(()),
        };

        self.base_strategy.on_fill(side, event.last_qty.as_f64());

        log::info!(
            "订单成交: {} {} @ {} | 库存: {:.4}",
            event.order_side,
            event.last_qty,
            event.last_px,
            self.base_strategy.get_stats().current_inventory
        );

        Ok(())
    }

    // 订单取消时调用
    fn on_order_canceled(&mut self, event: &OrderCanceled) -> Result<()> {
        log::info!("订单取消: {}", event.client_order_id);
        Ok(())
    }
}

// Deref trait 实现，以便策略可以直接访问 DataActorCore 的方法
impl Deref for NautilusAvellanedaStoikov {
    type Target = DataActorCore;

    fn deref(&self) -> &Self::Target {
        &self.core.actor
    }
}

impl DerefMut for NautilusAvellanedaStoikov {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core.actor
    }
}

/// 创建策略实例的辅助函数
pub fn create_strategy() -> NautilusAvellanedaStoikov {
    let base_config = ASConfig::default();
    NautilusAvellanedaStoikov::from_base_config(base_config)
}

/// 使用自定义配置创建策略实例
pub fn create_strategy_with_config(config: ASConfig) -> NautilusAvellanedaStoikov {
    NautilusAvellanedaStoikov::from_base_config(config)
}
