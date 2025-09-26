use hyperliquid_rust_sdk::{BaseUrl, InfoClient};

use crate::crypto::{ports::port::Port, types::Symbol};

/// hyperliquid
pub struct Hyperliquid {
    pub client: InfoClient,
}

impl Hyperliquid {
    pub async fn new() -> Self {
        Self {
            client: InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap(),
        }
    }
    fn get_token_price(&self) {
        todo!()
    }
}

impl Port for Hyperliquid {
    fn get_price(&self, symbol: Symbol) -> f64 {
        todo!()
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
