# happypto
a comprehensive SDK in the blockchain field, the functions will cover all common needs in the current blockchain field, for all chains. 
# Other List
1. happypto java sdk [Go](https://github.com/0xhappyboy/happypto-j/tree/main)
# Configuration
## Cargo.toml
```
[dependencies]
happypto = {git = "https://github.com/0xhappyboy/happypto", branch = "main"}
tokio = {version = "1.4", features = ["full"]}

[patch.crates-io]
merlin = {git = "https://github.com/aptos-labs/merlin"}
x25519-dalek = {git = "https://github.com/aptos-labs/x25519-dalek", branch = "zeroize_v1"}
```
## .cargo/config.toml
```
[build]
rustflags = ["--cfg", "tokio_unstable"]
```
# Module
```
ethereum::network : handle ethereum network functions
ethereum::wallet : ethereum wallet processing
ethereum::utils : related tools
ethereum::scout : ethereum network reconnaissance function

solana::scout : soalna network reconnaissance function
solana::network : handle solana network functions
solana::wallet : solana wallet processing
solana::config : overall situation config
solana::utils : related tools

aptos::account: provides operations for accounts
aptos::client : used to initialize the client and oper
aptos::config : overall situation config
aptos::transfer : provides methods for transactions
aptos::net : provides methods for net
aptos::coin : provides methods for coin
aptos::net : provides methods for net
aptos::faucet : faucet related methods, valid in Mode::DEV Mode::TEST mode
aptos::utils : internal utils
```