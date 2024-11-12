//! overall situation config
use std::str::FromStr;

use once_cell::sync::Lazy;
use url::Url;

pub static SOLANA_DEV_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://api.devnet.solana.com")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.devnet.solana.com"),
    )
    .unwrap()
});
pub static SOLANA_TEST_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://api.testnet.solana.com")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.testnet.solana.com"),
    )
    .unwrap()
});
pub static SOLANA_MAIN_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://api.mainnet-beta.solana.com")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.mainnet-beta.solana.com"),
    )
    .unwrap()
});
