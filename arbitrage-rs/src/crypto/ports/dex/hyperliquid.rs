use hyperliquid_rust_sdk::{BaseUrl, InfoClient};

use crate::crypto::ports::port::Port;

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
    fn get_price(&self) {
        todo!()
    }

    fn buy(&self) {
        todo!()
    }

    fn sell(&self) {
        todo!()
    }
}
