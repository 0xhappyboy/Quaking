/// crypto-related custom types

/// evm token price type
#[derive(Debug, Clone)]
pub struct EvmPriceType {
    usd: f64,
    eth: f64,
}
impl EvmPriceType {
    pub fn new(usd: f64, eth: f64) -> Self {
        Self { usd: usd, eth: eth }
    }
    pub fn usd(&self) -> f64 {
        self.usd
    }
    pub fn eth(&self) -> f64 {
        self.eth
    }
}

// KLine time type
pub const S_1: &str = "1s";
pub const MIN_1: &str = "1m";
pub const MIN_3: &str = "3m";
pub const MIN_5: &str = "5m";
pub const MIN_15: &str = "15m";
pub const MIN_30: &str = "30m";
pub const H_1: &str = "1h";
pub const H_2: &str = "2h";
pub const H_4: &str = "4h";
pub const H_6: &str = "6h";
pub const H_8: &str = "8h";
pub const H_12: &str = "12h";
pub const D_1: &str = "1d";
pub const D_3: &str = "3d";
pub const W_1: &str = "1w";
pub const M_1: &str = "1M";

// trade symbol(trading pair)
pub struct Symbol {
    pub symbol: String,
}
impl Symbol {
    pub fn btc_usdt() -> Self {
        Self {
            symbol: "BTCUSDT".to_string(),
        }
    }
    pub fn eth_usdt() -> Self {
        Self {
            symbol: "ETHUSDT".to_string(),
        }
    }
    pub fn sol_usdt() -> Self {
        Self {
            symbol: "SOLUSDT".to_string(),
        }
    }
    pub fn bnb_usdt() -> Self {
        Self {
            symbol: "BNBUSDT".to_string(),
        }
    }
}
