use hyperliquid_rust_sdk::{BaseUrl, InfoClient};

/// Defines relevant transaction service ports, such as well-known centralized exchanges and decentralized exchanges.
/// base port
pub trait Port {
    fn get_price(&self);
    fn buy(&self);
    fn sell(&self);
}

/// binance
pub struct Binance {}

impl Binance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Port for Binance {
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
    fn get_(&self) {
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

/// lighter
struct Lighter {}

impl Lighter {}

impl Port for Lighter {
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

/// bybit
struct Bybit {}

impl Bybit {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Bybit {
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

/// upbit
struct Upbit {}

impl Upbit {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Upbit {
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
/// okx
struct OKX {}

impl OKX {
    fn get_(&self) {
        todo!()
    }
}
impl Port for OKX {
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
/// bitget
struct Bitget {}

impl Bitget {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Bitget {
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
/// mexc
struct Mexc {}

impl Mexc {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Mexc {
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
/// gate
struct Gate {}

impl Gate {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Gate {
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
/// coinbase
struct Coinbase {}

impl Coinbase {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Coinbase {
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
/// kraken
struct Kraken {}

impl Kraken {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Kraken {
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
/// kucoin
struct Kucoin {}

impl Kucoin {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Kucoin {
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
/// htx
struct HTX {}

impl HTX {
    fn get_(&self) {
        todo!()
    }
}
impl Port for HTX {
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
/// crypto
struct Crypto {}

impl Crypto {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Crypto {
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
