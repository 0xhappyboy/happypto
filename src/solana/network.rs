use eyre::Ok;
use eyre::Result;
use solana_client::rpc_client::RpcClient;

use super::config::SOLANA_DEV_NET_URL;
use super::config::SOLANA_MAIN_NET_URL;
use super::config::SOLANA_TEST_NET_URL;

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
        match mode {
            Mode::MAIN => Ok(Self {
                mode,
                c: Some(RpcClient::new(SOLANA_MAIN_NET_URL.to_string())),
            }),
            Mode::TEST => Ok(Self {
                mode: mode,
                c: Some(RpcClient::new(SOLANA_TEST_NET_URL.to_string())),
            }),
            Mode::DEV => Ok(Self {
                mode: mode,
                c: Some(RpcClient::new(SOLANA_DEV_NET_URL.to_string())),
            }),
            _ => Ok(Self {
                mode: mode,
                c: None,
            }),
        }
    }
}
