//! faucet related methods, valid in Mode::DEV Mode::TEST mode
use aptos_sdk::types::LocalAccount;

use super::{client, config::FAUCET_CLIENT, utils};

/// use the designated account to obtain faucet tokens
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// let mut account = create_new_account();
/// faucet::get_faucet_coin(&aptos_client,account,10)
/// ```
pub async fn get_faucet_coin(
    aptos_client: &client::AptosClient,
    account: &LocalAccount,
    coin_amount: f64,
) {
    let _ = FAUCET_CLIENT
        .fund(account.address(), utils::wrap_coin_amount(coin_amount))
        .await;
}
