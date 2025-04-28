//! overall situation config
use std::str::FromStr;

use once_cell::sync::Lazy;
use url::Url;

pub const SOLANA_DEV_NET_URL: &'static str = "https://api.devnet.solana.com";
pub const SOLANA_TEST_NET_URL: &'static str = "https://api.testnet.solana.com";
pub const SOLANA_MAIN_NET_URL: &'static str = "https://api.mainnet-beta.solana.com";
