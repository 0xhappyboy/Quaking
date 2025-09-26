use binance::{
    account::Account,
    market::Market,
    model::{Prices, SymbolPrice},
};

use crate::crypto::{ports::port::Port, types::Symbol};

/// binance
pub struct Binance {
    pub market: Option<Market>,
    pub account: Option<Account>,
}

impl Binance {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        if !api_key.is_none() && !secret_key.is_none() {
            Self {
                market: Some(binance::api::Binance::new(
                    api_key.clone(),
                    secret_key.clone(),
                )),
                account: Some(binance::api::Binance::new(
                    api_key.clone(),
                    secret_key.clone(),
                )),
            }
        } else {
            Self {
                market: Some(binance::api::Binance::new(None, None)),
                account: None,
            }
        }
    }
    pub fn get_all_prices(&self) -> Vec<SymbolPrice> {
        match self.market.as_ref().unwrap().get_all_prices() {
            Ok(answer) => match answer {
                Prices::AllPrices(v) => v,
            },
            Err(e) => vec![],
        };
        vec![]
    }
}

impl Port for Binance {
    fn get_price(&self, symbol: Symbol) -> f64 {
        self.market
            .as_ref()
            .unwrap()
            .get_price(symbol.symbol)
            .unwrap()
            .price
    }
    fn spot_buy(&self) {
        todo!()
    }

    fn spot_sell(&self) {
        todo!()
    }

    fn futures_long(&self) {
        todo!()
    }

    fn futures_short(&self) {
        todo!()
    }
}
