/// price difference processing is used to execute when price differences are found.
type PriceDiffHandle = fn(dyn BasePrice);

/// Price processing module
pub struct Price {}

impl Price {
    /// price difference discovery
    fn find_price_diff(self, price_diff_handle: Option<PriceDiffHandle>) {
        let p = BinancePrice::new();
        match price_diff_handle {
            Some(handle) => {
                handle(p);
            }
            None => todo!(),
        }
    }
}

trait BasePrice {
    fn new() -> Self;
    fn get_price();
    fn test() {}
}

/// binance price
struct BinancePrice {}

impl BasePrice for BinancePrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// hyperliquid price
struct HyperliquidPrice {}

impl BasePrice for HyperliquidPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// bybit price
struct BybitPrice {}

impl BasePrice for BybitPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// upbit price
struct UpbitPrice {}

impl BasePrice for UpbitPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// okx price
struct OKXPrice {}

impl BasePrice for OKXPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// bitget price
struct BitgetPrice {}

impl BasePrice for BitgetPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// mexc price
struct MexcPrice {}

impl BasePrice for MexcPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// gate price
struct GatePrice {}

impl BasePrice for GatePrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// coinbase price
struct CoinbasePrice {}

impl BasePrice for CoinbasePrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// kraken price
struct KrakenPrice {}

impl BasePrice for KrakenPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// kucoin price
struct KucoinPrice {}

impl BasePrice for KucoinPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// htx price
struct HTXPrice {}

impl BasePrice for HTXPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}

/// crypto price
struct CryptoPrice {}

impl BasePrice for CryptoPrice {
    fn new() -> Self {
        Self {}
    }
    fn get_price() {
        todo!()
    }
}
