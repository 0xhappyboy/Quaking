use crate::crypto::ports::port::Port;
use crate::crypto::types::Symbol;

/// aster
pub struct Aster {}

impl Aster {
    pub async fn new() -> Self {
        Self {}
    }
    fn get_token_price(&self) {
        todo!()
    }
}

impl Port for Aster {
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
