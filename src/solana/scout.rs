use std::str::FromStr;

use eyre::Ok;
use eyre::Result;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status::UiTransactionEncoding;

use super::network::Mode;
use super::network::SolanaNetwork;

pub struct Scout {
    mode: Mode,
    network: SolanaNetwork,
}

impl Scout {
    pub fn new(mode: Mode) -> Result<Scout> {
        let n = SolanaNetwork::new(mode).unwrap();
        Ok(Self {
            mode: mode,
            network: n,
        })
    }
    /// get the slot interval of the current epoch
    pub fn get_solt_limit_by_current_epoch(&self) -> (u64, u64) {
        let epoch = self.network.c.as_ref().unwrap().get_epoch_info().unwrap();
        (0, 0)
    }
    /// get signature history of specified address
    ///
    /// ```
    /// let scout = Scout::new(solana::network::Mode::MAIN).unwrap();
    /// let list = scout.get_signature_history_by_address("");
    /// ```
    pub fn get_signature_history_by_address(
        &self,
        address: &str,
    ) -> Vec<solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature> {
        self.network
            .c
            .as_ref()
            .unwrap()
            .get_signatures_for_address(&Pubkey::from_str_const(address))
            .unwrap()
    }
    /// parse transaction details by string
    ///
    /// ```
    /// let scout = Scout::new(solana::network::Mode::MAIN).unwrap();
    /// let list = scout.get_signature_history_by_address("");
    /// for t in list {
    ///    scout.parse_transaction_data_by_signature_str(&t.signature);
    /// }
    /// ```
    pub fn parse_transaction_data_by_signature_str(
        &self,
        signature: &str,
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let transaction_info: EncodedConfirmedTransactionWithStatusMeta = self
            .network
            .c
            .as_ref()
            .unwrap()
            .get_transaction(
                &Signature::from_str(signature).unwrap(),
                UiTransactionEncoding::Json,
            )
            .unwrap();
        transaction_info
    }
    /// get last block transaction list
    pub fn get_last_block(&self) {
        todo!()
    }
    /// get last block transaction list
    pub fn last_block_transaction_list(&self) {
        todo!()
    }
}

struct Transaction {
    raw: String,
}

impl Transaction {
    pub fn new(raw: String) -> Result<Self> {
        Ok(Self {
            raw: String::from(""),
        })
    }
}
