use alloy::{
    network::TransactionBuilder,
    primitives::{address, utils::format_units, Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    sol,
    sol_types::SolCall,
};
use alloy::{
    providers::RootProvider,
    transports::http::{Client, Http},
};
use eyre::Ok;
use eyre::Result;
use std::str::FromStr;

use super::utils::get_usd_value;

sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");

/// ethereum network abstraction
pub struct EthereumNetwork {
    provider: RootProvider<Http<Client>>,
}

impl EthereumNetwork {
    pub fn new(rpc: &str) -> Result<EthereumNetwork> {
        Ok(Self {
            provider: ProviderBuilder::new().on_http(rpc.trim().parse()?),
        })
    }
    pub async fn address_balance() -> Result<u64> {
        Ok(0)
    }
    pub async fn last_block(&self) -> Result<u64> {
        Ok(self.provider.get_block_number().await?)
    }
    pub fn reset_provider(&mut self, rpc: &str) -> Result<()> {
        self.provider = ProviderBuilder::new().on_http(rpc.trim().parse()?);
        Ok(())
    }
    // get gas, unit: USD
    pub async fn get_gas(&mut self) -> Result<f64> {
        let call = latestAnswerCall {}.abi_encode();
        let input = Bytes::from(call);
        let tx = TransactionRequest::default()
            .with_to(ETH_USD_FEED)
            .with_input(input);
        let response = self.provider.call(&tx).await?;
        let result = U256::from_str(&response.to_string())?;
        let wei_per_gas = self.provider.get_gas_price().await?;
        let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
        Ok(gwei)
    }
    // get gas, unit: USD
    pub async fn get_gas_usd(&mut self) -> Result<f64> {
        let call = latestAnswerCall {}.abi_encode();
        let input = Bytes::from(call);
        let tx = TransactionRequest::default()
            .with_to(ETH_USD_FEED)
            .with_input(input);
        let response = self.provider.call(&tx).await?;
        let result = U256::from_str(&response.to_string())?;
        let wei_per_gas = self.provider.get_gas_price().await?;
        let usd = get_usd_value(wei_per_gas, result)?;
        Ok(usd)
    }
}
