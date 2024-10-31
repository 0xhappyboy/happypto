pub enum Chain {
    Ethereum,
    Solana,
    Aptos,
    Sui,
}

use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::{Client, Http},
};
use eyre::{Ok, Result};
/// Ethereum network abstraction
pub struct Ethereum {
    provider: RootProvider<Http<Client>>,
}
impl Ethereum {
    pub fn new(rpc: &str) -> Result<Ethereum> {
        Ok(Self {
            provider: ProviderBuilder::new().on_http(rpc.trim().parse()?),
        })
    }
    pub async fn address_balance() -> Result<u64> {
        Ok(0)
    }
    pub async fn last_block(&self) -> Result<u64> {
        Ok(self.provider.get_block_number().await?)
    }
    pub fn reset_provider(&mut self, rpc: &str) -> Result<()> {
        self.provider = ProviderBuilder::new().on_http(rpc.trim().parse()?);
        Ok(())
    }
}

/// Solana network abstraction
pub struct Solana {
    rpc: String,
}
impl Solana {
    pub fn new(rpc: String) -> Self {
        Self { rpc: rpc }
    }
    pub fn last_block(&self) -> u64 {
        0
    }
}

/// Aptos network abstraction
pub struct Aptos {
    rpc: String,
}
impl Aptos {
    pub fn new(rpc: String) -> Self {
        Self { rpc: rpc }
    }
    pub fn last_block(&self) -> u64 {
        0
    }
}
