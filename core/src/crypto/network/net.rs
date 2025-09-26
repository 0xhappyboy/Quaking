use crate::crypto::network::{evm::Evm, solana::Solana};

pub struct Network {
    pub ethereum: Evm,
    pub bsc: Evm,
    pub base: Evm,
    pub arb: Evm,
    pub hyper_evm: Evm,
    pub plasma: Evm,
    pub solana: Solana,
}

impl Network {
    pub async fn new() -> Self {
        Self {
            ethereum: Evm::new(super::evm::EvmNetworkType::Ethereum).await,
            bsc: Evm::new(super::evm::EvmNetworkType::Bsc).await,
            base: Evm::new(super::evm::EvmNetworkType::Base).await,
            arb: Evm::new(super::evm::EvmNetworkType::Arb).await,
            hyper_evm: Evm::new(super::evm::EvmNetworkType::HyperEVM).await,
            plasma: Evm::new(super::evm::EvmNetworkType::Plasma).await,
            solana: Solana::new(super::solana::Mode::MAIN).unwrap(),
        }
    }
}
