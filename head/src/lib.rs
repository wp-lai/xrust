pub fn parse_positive_int(val: &str) -> Result<usize, &str> {
    match val.parse() {
        Ok(n) => Ok(n),
        _ => Err(val),
    }
}
