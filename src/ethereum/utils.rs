use alloy::primitives::{utils::format_units, U256};
use eyre::Ok;
use eyre::Result;

const ETH_USD_FEED_DECIMALS: u8 = 8;
const ETH_DECIMALS: u32 = 18;

pub fn get_usd_value(amount: u128, price_usd: U256) -> Result<f64> {
    let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value = U256::from(amount) * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;
    Ok(formatted)
}

pub fn au8_to_string(signature_code: Vec<u8>) -> String {
    let mut private_key = String::new();
    for a in signature_code.iter() {
        let fstr = format!("{:02x}", a);
        private_key.push_str(&fstr);
    }
    private_key
}
