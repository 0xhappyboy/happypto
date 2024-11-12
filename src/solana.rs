/// Solana network abstraction
pub struct Solana {
    rpc: String,
}
impl Solana {
    pub fn new(rpc: String) -> Self {
        Self { rpc: rpc }
    }
    pub fn last_block(&self) -> u64 {
        0
    }
}
