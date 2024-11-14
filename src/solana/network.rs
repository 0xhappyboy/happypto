/// solana network core, including rpc related operations and re-encapsulation of some key data structures
use crate::aptos::client;
use eyre::Result;
use log::error;
use log::info;
use serde::Deserialize;
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_client::rpc_response::Response;
use solana_sdk::epoch_info::EpochInfo;
use solana_transaction_status::EncodedConfirmedBlock;

use super::config::SOLANA_DEV_NET_URL;
use super::config::SOLANA_MAIN_NET_URL;
use super::config::SOLANA_TEST_NET_URL;
use super::scout::Scout;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    MAIN,
    TEST,
    DEV,
}

pub struct SolanaNetwork {
    mode: Mode,
    pub c: Option<RpcClient>,
}

impl SolanaNetwork {
    pub fn new(mode: Mode) -> Result<SolanaNetwork> {
        let mut url = String::new();
        match mode {
            Mode::MAIN => {
                url = SOLANA_MAIN_NET_URL.to_string();
            }
            Mode::TEST => {
                url = SOLANA_TEST_NET_URL.to_string();
            }
            Mode::DEV => {
                url = SOLANA_DEV_NET_URL.to_string();
            }
            _ => {
                return Ok(Self {
                    mode: mode,
                    c: None,
                });
            }
        }
        let client = RpcClient::new(url.clone());
        Ok(Self {
            mode,
            c: Some(client),
        })
    }
    ///get epoch
    pub fn get_epoch(&self) -> EpochInfo {
        self.c.as_ref().unwrap().get_epoch_info().unwrap()
    }
    /// get slot
    pub fn get_slot(&self) -> u64 {
        self.get_epoch().absolute_slot
    }
    /// get scout
    pub fn get_scout(&self) -> Result<Scout> {
        Scout::new(self.mode)
    }
    /// override of get_block function
    pub fn rpc_get_block(&self, slot: u64) -> Result<Block, String> {
        let params = json!([slot, {
            "encoding": "json",
            "transactionDetails": "full",
            "rewards": false,
            "maxSupportedTransactionVersion": 0
        }]);
        match self
            .c
            .as_ref()
            .unwrap()
            .send::<serde_json::Value>(RpcRequest::GetBlock, params)
        {
            Ok(r) => {}
            Err(err) => {
                error!(
                    "[SolanaNetwork::rpc_get_block] Error getting block data {:?}",
                    err
                )
            }
        }
        Err(String::from(""))
    }
}

impl AsRef<SolanaNetwork> for SolanaNetwork {
    fn as_ref(&self) -> &SolanaNetwork {
        self
    }
}

impl AsMut<SolanaNetwork> for SolanaNetwork {
    fn as_mut(&mut self) -> &mut SolanaNetwork {
        self
    }
}

/// block structure abstraction
#[derive(Debug, Clone, Deserialize)]
pub struct Block {
    pub blockHeight: u64,
    pub blockTime: u64,
    pub blockhash: String,
    pub parentSlot: u64,
    pub previousBlockhash: String,
    pub transactions: Vec<Transaction>,
}

impl AsRef<Block> for Block {
    fn as_ref(&self) -> &Block {
        self
    }
}

impl AsMut<Block> for Block {
    fn as_mut(&mut self) -> &mut Block {
        self
    }
}

/// transaction record data abstraction, symbolizing a transaction
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Transaction {
    meta: TransactionMeta,
}

impl Transaction {
    pub fn new(raw: String) -> Result<Self> {
        Ok(Transaction::default())
    }
}

impl AsRef<Transaction> for Transaction {
    fn as_ref(&self) -> &Transaction {
        self
    }
}

impl AsMut<Transaction> for Transaction {
    fn as_mut(&mut self) -> &mut Transaction {
        self
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TransactionMeta {
    #[serde(default)]
    pub fee: u64,
    #[serde(default)]
    pub logMessages: Vec<String>,
    #[serde(default)]
    pub postBalances: Vec<u64>,
    #[serde(default)]
    pub preBalances: Vec<u64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct TransactionMetaStatus {
    raw: Option<String>,
}
