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
