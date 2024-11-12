use super::utils::au8_to_string;
use eyre::Ok;
use eyre::Result;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Result<Self> {
        let k = Keypair::new();
        Ok(Self {
            private_key: au8_to_string(k.to_bytes().to_vec()),
            public_key: k.pubkey().to_string(),
        })
    }
}
