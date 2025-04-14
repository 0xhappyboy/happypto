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
    /// create new wallet cluster
    pub fn new_wallet_cluster(amount: u64) -> Result<Vec<Self>> {
        //let mut wallets = vec![];
        let mut wallets: Map<String, Wallet> = HashMap::new();
        for i in 0..amount {
            loop {
                let w = Wallet::new();
                if wallets.entry(w.unwrap().private_key) {
                    wallets.insert(w.unwrap().private_key, w.unwrap());
                    break;
                }
            }
        }
    }
    /// create new wallet cluster by private key
    pub fn new_wallet_cluster_private_keys(pks: Vec<String>) -> Result<Vec<Self>> {}
}
