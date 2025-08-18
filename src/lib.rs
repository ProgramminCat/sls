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

pub fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;

    if bytes_f >= GB {
        format!("{:.2}GB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2}MB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2}KB", bytes_f / KB)
    } else {
        format!("{}B", bytes)
    }
}