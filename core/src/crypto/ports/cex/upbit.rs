use crate::crypto::{ports::port::Port, types::Symbol};

/// upbit
struct Upbit {}

impl Upbit {
    fn get_(&self) {
        todo!()
    }
}
impl Port for Upbit {
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
