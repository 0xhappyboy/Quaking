/// evm network
use alloy::{
    network::TransactionBuilder,
    primitives::{Address, Bytes, U256, address, utils::format_units},
    providers::{Provider, ProviderBuilder, fillers::FillProvider},
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use std::str::FromStr;

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
const ETH_USD_FEED_DECIMALS: u8 = 8;
const ETH_DECIMALS: u32 = 18;
const EVM_RPC: &str = "https://reth-ethereum.ithaca.xyz/rpc";

/// smart contract function signature
sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);

#[derive(Clone)]
pub struct Evm {
    provider: FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::RootProvider,
    >,
}
impl Evm {
    pub async fn new() -> Self {
        let provider = ProviderBuilder::new().connect(EVM_RPC).await.unwrap();
        Self { provider: provider }
    }
    pub async fn get_gas_gwei(&self) -> f64 {
        let wei_per_gas = self.provider.get_gas_price().await.unwrap();
        let gwei = format_units(wei_per_gas, "gwei")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        gwei
    }
    pub async fn get_gas_usd(&self) -> f64 {
        let call = latestAnswerCall {}.abi_encode();
        let input = Bytes::from(call);
        let tx = TransactionRequest::default()
            .with_to(ETH_USD_FEED)
            .with_input(input);
        let response = self.provider.call(tx).await.unwrap();
        let result = U256::from_str(&response.to_string()).unwrap();
        let wei_per_gas = self.provider.get_gas_price().await.unwrap();
        let usd = get_usd_value(wei_per_gas, result);
        usd
    }
}
fn get_usd_value(amount: u128, price_usd: U256) -> f64 {
    let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value = U256::from(amount) * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)
        .unwrap()
        .parse::<f64>()
        .unwrap();
    formatted
}
