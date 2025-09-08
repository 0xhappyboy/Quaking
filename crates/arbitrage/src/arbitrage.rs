use crate::crypto::{
    ports::{Binance, Hyperliquid},
    price::Price,
};

pub struct Arbitrage {
    pub binance_port: Option<Binance>,
    pub hyperliquid_port: Option<Hyperliquid>,
}

impl Arbitrage {
    pub async fn init() -> Self {
        Self::new().await
    }
    pub async fn init_for_json() -> Self {
        Self::new().await
    }
    pub async fn init_for_json_file() -> Self {
        Self::new().await
    }
    pub async fn init_for_xml_file() -> Self {
        Self::new().await
    }
    pub async fn new() -> Self {
        Self {
            binance_port: None,
            hyperliquid_port: Some(Hyperliquid::new().await),
        }
    }
    pub fn price(self) -> Price {
        Price::new(self)
    }
}
