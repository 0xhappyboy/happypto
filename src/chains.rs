pub enum Chain {
    Ethereum,
    Solana,
    Aptos,
    Sui,
}

use alloy::{
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::{Client, Http},
};
use eyre::{Ok, Result};
