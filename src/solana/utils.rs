pub fn au8_to_string(signature_code: Vec<u8>) -> String {
    let mut private_key = String::new();
    for a in signature_code.iter() {
        let fstr = format!("{:02x}", a);
        private_key.push_str(&fstr);
    }
    private_key
}
