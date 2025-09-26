use crate::crypto::ports::port::Port;
use crate::crypto::types::Symbol;

/// okx
struct OKX {}

impl OKX {
    fn get_(&self) {
        todo!()
    }
}
impl Port for OKX {
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
