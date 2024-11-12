//! internal utils
pub fn wrap_coin_amount(coin_amount: f64) -> u64 {
    (coin_amount * (100_000_000 as f64)) as u64
}

pub fn unwrap_coin_amount(coin_amount: u64) -> f64 {
    ((coin_amount as f64) / (100_000_000 as f64))
}
