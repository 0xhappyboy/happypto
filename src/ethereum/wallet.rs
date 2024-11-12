use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use eyre::Ok;
use eyre::Result;

use super::utils::au8_to_string;

/// ethereum network wallet abstraction
pub struct Wallet {
    private_key: String,
    public_key: String,
    w: EthereumWallet,
    s: PrivateKeySigner,
}

impl Wallet {
    ///
    /// create new wallet
    ///
    /// ```
    /// let w = Wallet::new();
    /// ```
    pub fn new() -> Result<Wallet> {
        let signer = PrivateKeySigner::random();
        let ss = signer.clone().to_bytes();
        let asa = ss.as_slice();
        let wallet = EthereumWallet::from(signer.clone());
        Ok(Self {
            private_key: au8_to_string(asa.to_vec()),
            public_key: signer.clone().address().to_string(),
            w: wallet,
            s: signer,
        })
    }
    ///
    /// create new wallet by private key
    ///
    /// ```
    /// let w = Wallet::new_by_private_key(".....");
    /// ```
    pub fn new_by_private_key(private_key: &str) -> Result<Wallet> {
        let signer: PrivateKeySigner = private_key
            .parse()
            .expect("please enter the correct private key!");
        let wallet = EthereumWallet::from(signer.clone());
        Ok(Self {
            private_key: String::from(private_key),
            public_key: signer.clone().address().to_string(),
            w: wallet,
            s: signer,
        })
    }
    pub fn balance(&self) -> String {
        String::from("0")
    }
    pub fn private_key(&self) -> String {
        self.private_key.clone()
    }
    pub fn public_key(&self) -> String {
        self.public_key.clone()
    }
    pub fn w(&self) -> EthereumWallet {
        self.w.clone()
    }
    pub fn signer(&self) -> PrivateKeySigner {
        self.s.clone()
    }
}
