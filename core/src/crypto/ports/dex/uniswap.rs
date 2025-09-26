use crate::crypto::{network::evm::Evm, ports::port::Port, types::Symbol};

/// uniswap
pub struct UniSwap {
    evm: Evm,
}
impl UniSwap {
    pub fn new(evm: Evm) -> Self {
        Self { evm: evm }
    }
    /// Get the price of a specified token.
    pub fn get_token_price() -> u64 {
        0
    }
}
impl Port for UniSwap {
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
