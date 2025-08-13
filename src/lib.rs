pub fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();
    let num: f64 = s
        .chars()
        .take_while(|c| c.is_numeric() || *c == '.')
        .collect::<String>()
        .parse()
        .ok()?;
    if s.ends_with("KB") {
        Some((num * 1024.0) as u64)
    } else if s.ends_with("MB") {
        Some((num * 1024.0 * 1024.0) as u64)
    } else if s.ends_with("GB") {
        Some((num * 1024.0 * 1024.0 * 1024.0) as u64)
    } else {
        Some(num as u64)
    }
}