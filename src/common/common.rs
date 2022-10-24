pub fn get_current_milis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("ASD")
        .as_millis()
}