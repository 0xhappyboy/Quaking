use crate::crypto::{network::evm::Evm, ports::port::Port};

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
