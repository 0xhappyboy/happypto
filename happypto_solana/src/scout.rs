use std::str;
/// solana network scout, includes some useful solana network scout capabilities
use std::str::FromStr;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;

use eyre::Result;
use log::error;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use solana_transaction_status::UiTransactionEncoding;

use super::network::Block;
use super::network::Mode;
use super::network::SolanaNetwork;
use super::network::Transaction;

/// block scan interval (ms)
const BLOCK_SCAN_INTERVAL: u64 = 400;

/// block data handle function type
type FnBlockDataHandle = fn(&Scout, Block);
/// transaction data handle function type
type FnTradeDataHandle = fn(&Scout, Transaction);

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
    /// Example
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
    /// Example
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
    /// handle last block
    pub fn handle_last_block(
        &self,
        block_hanlde: Option<FnBlockDataHandle>,
        trade_hanlde: Option<FnTradeDataHandle>,
    ) {
        // thread-safe slot temporary storage
        let mut slot_storage = AtomicU64::new(0);
        let slot = self.network.as_ref().get_slot();
        if slot != slot_storage.load(Ordering::SeqCst) {
            match self.network.rpc_get_block(slot) {
                Ok(block) => {
                    if !block_hanlde.is_none() {
                        block_hanlde.unwrap()(self, block.clone());
                    }
                    if !trade_hanlde.is_none() {
                        for t in block.transactions {
                            // handle trade
                            todo!()
                        }
                    }
                }
                Err(e) => error!("{}", e),
            }
        }
    }
    /// polling to process the transaction data of the last block
    ///
    /// # Params
    ///
    /// - `block_hanlde` : block data processing function type
    /// - `trade_hanlde` : trade data processing function type
    ///
    /// Example
    /// ```
    /// fn block_handle(scount: &Scout, t: Transaction){
    ///    // .....
    /// }
    ///
    /// fn trade_handle(scount: &Scout, t: Transaction){
    ///    // .....
    /// }
    ///
    /// let scout = Scout::new(Mode::MAIN).unwrap();
    /// scout.poll_handle_transaction_of_last_block(block_handle,trade_handle);
    /// ```
    ///
    pub fn poll_last_block(
        &self,
        block_hanlde: Option<FnBlockDataHandle>,
        trade_hanlde: Option<FnTradeDataHandle>,
    ) {
        // thread-safe slot temporary storage
        let mut slot_storage = AtomicU64::new(0);
        loop {
            let slot = self.network.as_ref().get_slot();
            if slot != slot_storage.load(Ordering::SeqCst) {
                match self.network.rpc_get_block(slot) {
                    Ok(block) => {
                        if !block_hanlde.is_none() {
                            block_hanlde.unwrap()(self, block.clone());
                        }
                        if !trade_hanlde.is_none() {
                            for t in block.transactions {
                                // handle trade
                                todo!()
                            }
                        }
                    }
                    Err(e) => error!("{}", e),
                }
            } else {
                continue;
            }
            slot_storage.store(slot, Ordering::SeqCst);
            sleep(Duration::from_millis(BLOCK_SCAN_INTERVAL));
        }
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

/// filters for block data structures
pub struct BlockFilter {}
impl BlockFilter {}

/// filters for transaction data structures
pub struct TransactionFilter {}
impl TransactionFilter {}
