//! overall situation config
use std::str::FromStr;

use aptos_sdk::rest_client::FaucetClient;
use once_cell::sync::Lazy;
use url::Url;

pub static APTOS_DEV_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://fullnode.devnet.aptoslabs.com/v1")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com/v1"),
    )
    .unwrap()
});
pub static APTOS_TEST_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://testnet.aptoslabs.com/v1")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://testnet.aptoslabs.com/v1"),
    )
    .unwrap()
});
pub static APTOS_MAIN_NET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://fullnode.mainnet.aptoslabs.com/v1")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.mainnet.aptoslabs.com/v1"),
    )
    .unwrap()
});
pub static APTOS_FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("https://faucet.devnet.aptoslabs.com")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://faucet.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// faucet client
pub static FAUCET_CLIENT: Lazy<FaucetClient> =
    Lazy::new(|| FaucetClient::new(APTOS_FAUCET_URL.clone(), APTOS_TEST_NET_URL.clone()));
