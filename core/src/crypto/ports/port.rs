use crate::crypto::types::Symbol;

/// Defines relevant transaction service ports, such as well-known centralized exchanges and decentralized exchanges.
/// base port
pub trait Port {
    fn get_price(&self, symbol: Symbol) -> f64;
    fn spot_buy(&self);
    fn spot_sell(&self);
    fn futures_long(&self);
    fn futures_short(&self);
}
