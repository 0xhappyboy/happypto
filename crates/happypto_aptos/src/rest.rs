use super::{
    client::{AptosClient, Mode},
    config,
};

pub struct AptosRestApi {
    url: String,
}

impl AptosRestApi {
    pub fn new(aptos_client: &AptosClient) -> Self {
        match aptos_client.mode() {
            Mode::DEV => {
                return Self {
                    url: config::APTOS_DEV_NET_URL.to_string(),
                };
            }
            Mode::TEST => {
                return Self {
                    url: config::APTOS_TEST_NET_URL.to_string(),
                };
            }
            Mode::MAIN => {
                return Self {
                    url: config::APTOS_MAIN_NET_URL.to_string(),
                };
            }
        }
    }
    /// get network info
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_network_info().await;
    /// ```
    pub async fn get_network_info(&self) {
        let body = reqwest::get(self.url.clone())
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("body: {}", body);
    }
    /// get address info
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_address_info("0x...").await;
    /// ```
    pub async fn get_address_info(&self, address: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/accounts/{}", address));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get address modules
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_address_modules("0x...").await;
    /// ```
    pub async fn get_address_modules(&self, address: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/accounts/{}/modules", address));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get address module info by name
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_address_module_info("0x...","name").await;
    /// ```
    pub async fn get_address_module_info(&self, address: &str, name: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/accounts/{}/modules/{}", address, name));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get address resources
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_address_resources("0x...").await;
    /// ```
    pub async fn get_address_resources(&self, address: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/accounts/{}/resources", address));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get address resource type info by type
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_address_resource_info("0x...","resource_type").await;
    /// ```
    pub async fn get_address_resource_info(&self, address: &str, resource_type: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!(
            "/accounts/{}/resources/{}",
            address, resource_type
        ));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get block information at a specified height
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_block_info_by_height(1).await;
    /// ```
    pub async fn get_block_info_by_height(&self, block_height: u64) {
        let mut url = self.url.clone();
        url.push_str(&format!("/blocks/by_height/{}", block_height));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get block information at a specified height
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_block_info_by_version(1).await;
    /// ```
    pub async fn get_block_info_by_version(&self, version: u64) {
        let mut url = self.url.clone();
        url.push_str(&format!("/blocks/by_version/{}", version));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get spec
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.getspec().await;
    /// ```
    pub async fn getspec(&self) {
        let mut url = self.url.clone();
        url.push_str(&format!("/spec"));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// examine health
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.health().await;
    /// ```
    pub async fn health(&self) {
        let mut url = self.url.clone();
        url.push_str(&format!("/healthy"));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get transactions
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.transactions().await;
    /// ```
    pub async fn transactions(&self) {
        let mut url = self.url.clone();
        url.push_str(&format!("/transactions"));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get transaction info by hash
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_transaction_by_hash("0x").await;
    /// ```
    pub async fn get_transaction_by_hash(&self, hash: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/transactions/by_hash/{}", hash));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// get transaction records by address
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.get_transactions_by_address("0x").await;
    /// ```
    pub async fn get_transactions_by_address(&self, address: &str) {
        let mut url = self.url.clone();
        url.push_str(&format!("/accounts/{}/transactions", address));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
    /// estimate gas price
    ///
    /// # Examples
    ///
    /// ```
    /// let aptos_client = AptosClient::new(Mode::DEV);
    /// let rest_api: rest::AptosRestApi = rest::AptosRestApi::new(&aptos_client);
    /// rest_api.estimate_gas_price().await;
    /// ```
    pub async fn estimate_gas_price(&self) {
        let mut url = self.url.clone();
        url.push_str(&format!("/estimate_gas_price"));
        let body = reqwest::get(url).await.unwrap().text().await.unwrap();
        println!("body: {}", body);
    }
}
