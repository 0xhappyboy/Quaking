use crate::crypto::{
    network::net::Network,
    ports::{
        cex::binance::Binance,
        dex::{hyperliquid::Hyperliquid, uniswap::UniSwap},
    },
    price::Price,
};
use alloy::providers::{Provider, ProviderBuilder};

pub struct Arbitrage {
    pub binance_port: Option<Binance>,
    pub hyperliquid_port: Option<Hyperliquid>,
    pub uniswap_port: Option<UniSwap>,
    pub network: Option<Network>,
}

impl Arbitrage {
    pub async fn init() -> Self {
        Self::new().await
    }
    pub async fn init_by_json() -> Self {
        Self::new().await
    }
    pub async fn init_by_json_file() -> Self {
        Self::new().await
    }
    pub async fn init_by_xml_file() -> Self {
        Self::new().await
    }
    pub async fn new() -> Self {
        let net = Network::new().await;
        Self {
            binance_port: None,
            hyperliquid_port: Some(Hyperliquid::new().await),
            uniswap_port: Some(UniSwap::new(net.ethereum.clone())),
            network: Some(net),
        }
    }
    pub fn price(self) -> Price {
        Price::new(self)
    }
}
