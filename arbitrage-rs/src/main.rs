use alloy::primitives::Address;
use hyperliquid_rust_sdk::{BaseUrl, InfoClient};
use log::info;

use crate::arbitrage::Arbitrage;

mod arbitrage;
mod crypto;
const ADDRESS: &str = "0xc64cc00b46101bd40aa1c3121195e85c0b0918d8";

#[tokio::main]
async fn main() {
    env_logger::init();
    // let info_client: InfoClient = InfoClient::new(None, Some(BaseUrl::Testnet)).await.unwrap();
    let a = Arbitrage::init().await;
    a.price().test().await;
}
