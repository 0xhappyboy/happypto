/// Aptos network abstraction
pub struct Aptos {
    rpc: String,
}
impl Aptos {
    pub fn new(rpc: String) -> Self {
        Self { rpc: rpc }
    }
    pub fn last_block(&self) -> u64 {
        0
    }
}
