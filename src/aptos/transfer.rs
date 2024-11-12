//! Provides methods for transactions
use anyhow::{Context, Ok};
use aptos_sdk::{
    coin_client::{CoinClient, TransferOptions},
    rest_client::{Client, PendingTransaction},
    types::LocalAccount,
};

use super::{client::AptosClient, utils::wrap_coin_amount};

pub struct ChainId {
    chain_id: u8,
}

impl ChainId {
    pub async fn new(aptos_client: &AptosClient) -> Result<Self, anyhow::Error> {
        Ok(Self {
            chain_id: aptos_client
                .rest_client()
                .clone()
                .unwrap()
                .get_index()
                .await
                .context("failed to get chain id")?
                .inner()
                .chain_id,
        })
    }
}

/// transfer structure options
#[derive(Debug, Clone)]
pub struct TransactionOptions {
    ///
    pub max_gas_amount: u64,
    pub gas_unit_price: u64,
    pub timeout_sec: u64,
    pub coin_type: String,
}

impl Default for TransactionOptions {
    fn default() -> Self {
        Self {
            max_gas_amount: 5_000,
            gas_unit_price: 100,
            timeout_sec: 10,
            coin_type: "0x1::aptos_coin::AptosCoin".to_string(),
        }
    }
}

/// all information required during the transaction process
pub struct Transaction {
    from: LocalAccount,
    to: LocalAccount,
    amount: f64,
    transaction_options: TransactionOptions,
}

impl Transaction {
    /// create a new transaction object
    pub async fn new(
        aptos_client: &AptosClient,
        f: LocalAccount,
        t: LocalAccount,
        a: f64,
        o: Option<TransactionOptions>,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            from: f,
            to: t,
            amount: a,
            transaction_options: o.unwrap_or_default(),
        })
    }
    /// create a txn hash
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub async fn create_txn_hash(&mut self, aptos_client: &AptosClient) -> PendingTransaction {
        let rest_client: Client = aptos_client.rest_client().clone().unwrap();
        let coin_client = CoinClient::new(&rest_client);
        let txn_hash: Result<aptos_sdk::rest_client::PendingTransaction, anyhow::Error> =
            coin_client
                .transfer(
                    &mut self.from,
                    self.to.address(),
                    wrap_coin_amount(self.amount),
                    None,
                )
                .await;
        txn_hash.unwrap()
    }
    /// send a txn hash
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub async fn send_txn_hash(&self, aptos_client: &AptosClient, txn_hash: PendingTransaction) {
        let rest_client: Client = aptos_client.rest_client().clone().unwrap();
        let _ = rest_client.wait_for_transaction(&txn_hash).await;
    }
    /// batch transfer
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub async fn send(&mut self, aptos_client: &AptosClient) {
        let txn = self.create_txn_hash(&aptos_client).await;
        self.send_txn_hash(&aptos_client, txn).await;
    }
    /// batch transfer
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    pub async fn batch_send(&mut self, aptos_client: &AptosClient, to: &Vec<&mut LocalAccount>) {
        for (_index, to_accout) in to.iter().enumerate() {
            let rest_client: Client = aptos_client.rest_client().clone().unwrap();
            let coin_client = CoinClient::new(&rest_client);
            let txn_hash = coin_client
                .transfer(
                    &mut self.from,
                    to_accout.address(),
                    wrap_coin_amount(self.amount),
                    Some(TransferOptions::default()),
                )
                .await;
            println!("{:?}", txn_hash);
            self.send_txn_hash(&aptos_client, txn_hash.unwrap()).await;
        }
    }
}
