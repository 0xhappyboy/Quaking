/// Defines relevant transaction service ports, such as well-known centralized exchanges and decentralized exchanges.
/// base port
pub trait Port {
    fn get_price(&self);
    fn buy(&self);
    fn sell(&self);
}
