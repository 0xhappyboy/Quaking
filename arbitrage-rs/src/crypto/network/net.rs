use crate::crypto::network::{evm::Evm, solana::Solana};

#[derive(Clone)]
pub struct Network {
    pub evm: Evm,
    pub solana: Solana,
}
impl Network {
    pub async fn new() -> Self {
        Self {
            evm: Evm::new().await,
            solana: Solana::new(),
        }
    }
}
