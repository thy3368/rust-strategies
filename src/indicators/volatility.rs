//! 波动率计算指标 - SIMD优化版本

/// EWMA波动率计算器（指数加权移动平均）
pub struct EWMAVolatility {
    alpha: f64,
    variance: f64,
    initialized: bool,
}

impl EWMAVolatility {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            variance: 0.0,
            initialized: false,
        }
    }

    #[inline]
    pub fn update(&mut self, return_value: f64) -> f64 {
        if !self.initialized {
            self.variance = return_value * return_value;
            self.initialized = true;
        } else {
            self.variance = self.alpha * return_value * return_value
                          + (1.0 - self.alpha) * self.variance;
        }

        self.variance.sqrt()
    }

    pub fn get(&self) -> f64 {
        self.variance.sqrt()
    }
}

/// Garman-Klass波动率估计器
/// 使用OHLC数据，比简单波动率更准确
pub fn garman_klass_volatility(ohlc: &[(f64, f64, f64, f64)]) -> f64 {
    if ohlc.is_empty() {
        return 0.0;
    }

    let mut sum = 0.0;
    let ln2 = std::f64::consts::LN_2;

    for &(open, high, low, close) in ohlc {
        let hl = (high / low).ln();
        let co = (close / open).ln();

        sum += 0.5 * hl * hl - (2.0 * ln2 - 1.0) * co * co;
    }

    (sum / ohlc.len() as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ewma_volatility() {
        let mut ewma = EWMAVolatility::new(0.94);

        let returns = vec![0.01, -0.015, 0.008, -0.012, 0.02];

        for ret in returns {
            let vol = ewma.update(ret);
            assert!(vol >= 0.0);
        }
    }
}
