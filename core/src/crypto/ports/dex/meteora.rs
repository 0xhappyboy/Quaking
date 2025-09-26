use crate::crypto::{ports::port::Port, types::Symbol};

/// meteora
struct Meteora {}
impl Meteora {}
impl Port for Meteora {
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
