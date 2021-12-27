
pub fn bool_to_string(b: bool) -> String {
    match b {
        true => "1".to_string(),
        false => "0".to_string()
    }
}
