use crate::crypto::ports::port::Port;

// ============== CEX ==============
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