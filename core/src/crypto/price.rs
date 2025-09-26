use core::sync;

use crate::{
    arbitrage::{self, Arbitrage},
    crypto::{global::TradeTypeEnum, ports::port::Port},
};

/// list of ports with price differences.
type PriceDiffHandle = fn(Vec<Box<&dyn Port>>);
/// best price port.
type BestPricePort = fn(Box<dyn Port>);

/// Price processing module
pub struct Price {
    pub arbitrage: Arbitrage,
}

impl Price {
    pub fn new(arbitrage: Arbitrage) -> Self {
        Self {
            arbitrage: arbitrage,
        }
    }
    /// price difference discovery
    pub async fn find_price_diff(&self, coin: String, price_diff_handle: Option<PriceDiffHandle>) {
        /// test ......
        let a = self.arbitrage.hyperliquid_port.as_ref().unwrap();
        let list: Vec<Box<&dyn Port>> = vec![Box::new(a)];
        match price_diff_handle {
            Some(handle) => handle(list),
            None => todo!(),
        }
    }
    /// best price
    pub async fn find_best_price(
        &self,
        coin: String,
        trade_type: TradeTypeEnum,
        best_price_handle: Option<BestPricePort>,
    ) {
        match best_price_handle {
            Some(handle) => match trade_type {
                TradeTypeEnum::Buy => {
                    // Find the cheapest price among all ports
                }
                TradeTypeEnum::Sell => {
                    //Find the most expensive port among all ports
                }
            },
            None => todo!(),
        }
    }
    /// test
    pub async fn test(&self) {
        println!(
            "meta data {:?}",
            self.arbitrage
                .hyperliquid_port
                .as_ref()
                .unwrap()
                .client
                .meta()
                .await
        );
    }
}
