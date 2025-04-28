//! Provides operations for accounts
use aptos_sdk::{
    coin_client::CoinClient, crypto::ValidCryptoMaterialStringExt, rest_client::Client,
    types::LocalAccount,
};

use super::{client::AptosClient, utils};

/// create a account
///
/// # Examples
///
/// ```
/// create_new_account()
/// ```
pub fn create_new_account() -> LocalAccount {
    let account = LocalAccount::generate(&mut rand::rngs::OsRng);
    account
}

/// create a account by private key
///
/// # Examples
///
/// ```
/// create_account_by_private_key(private_key)
/// ```
pub fn create_account_by_private_key(private_key: &str) -> LocalAccount {
    let account = LocalAccount::from_private_key(private_key, 0).unwrap();
    account
}

/// create a vanity account
///
/// # Examples
///
/// ```
/// let aptos_client = AptosClient::new(Mode::DEV);
/// create_vanity_account("6666".to_string(),"8888".to_string())
/// ```
pub fn create_vanity_account(start: &str, end: &str) -> LocalAccount {
    // create new account
    let mut account: LocalAccount = create_new_account();
    loop {
        // get public key string
        let mut public_key_string = get_public_key(&account);
        public_key_string = public_key_string.replace("0x", "");
        if (public_key_string.starts_with(start) && public_key_string.ends_with(end)) {
            // to meet the conditions
            break;
        } else {
            account = create_new_account();
        }
    }
    account
}

/// get the public key string of an account
///
/// # Examples
///
/// ```
/// get_public_key(account)
/// ```
pub fn get_public_key(account: &LocalAccount) -> String {
    account.address().to_hex_literal()
}

/// get the private key string of an account
///
/// # Examples
///
/// ```
/// get_private_key(account)
/// ```
pub fn get_private_key(account: &LocalAccount) -> String {
    account.private_key().to_encoded_string().unwrap()
}

/// get account balance
///
/// # Examples
///
/// ```
/// get_account_balance(account)
/// ```
pub async fn get_account_balance(aptos_client: &AptosClient, account: &LocalAccount) -> f64 {
    let rest_client: Client = aptos_client.rest_client().clone().unwrap();
    let coin_client = CoinClient::new(&rest_client);
    match coin_client.get_account_balance(&account.address()).await {
        Result::Ok(b) => return utils::unwrap_coin_amount(b),
        Result::Err(e) => {
            println!("get account balance error, e={}", e);
            return 0.0;
        }
    }
}
