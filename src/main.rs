use clap::Parser;
use colored::*;
use glob::Pattern;
use serde::Serialize;
use walkdir::WalkDir;
use chrono::{DateTime, NaiveDate, Utc, Local};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<String>,

    #[arg(long)]
    ext: Option<String>,

    /// Minimum file size (example: 10KB, 1MB)
    #[arg(long)]
    min_size: Option<String>,

    /// Maximum file size
    #[arg(long)]
    max_size: Option<String>,

    /// Include hidden files
    #[arg(long)]
    hidden: bool,

    /// Modified date range: example: "2025-01-01..2025-08-10"
    #[arg(long)]
    modified: Option<String>,

    /// Pattern include (example: "*.rs")
    #[arg(long)]
    include: Option<String>,

    /// Pattern exclude (example: "*test*")
    #[arg(long)]
    exclude: Option<String>,

    /// Output JSON instead
    #[arg(long)]
    json: bool,

    /// Max depth for recursion (default unlimited)
    #[arg(long)]
    depth: Option<usize>,
}

#[derive(Serialize)]
struct FileInfo {
    path: String,
    size: u64,
    modified: Option<String>,
    is_dir: bool,
    permissions: String,
}

fn parse_size(s: &str) -> Option<u64> {
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

fn parse_date_range(range: &str) -> Option<(NaiveDate, NaiveDate)> {
    let parts: Vec<&str> = range.split("..").collect();
    if parts.len() != 2 {
        return None;
    }
    let start = NaiveDate::parse_from_str(parts[0].trim(), "%Y-%m-%d").ok()?;
    let end = NaiveDate::parse_from_str(parts[1].trim(), "%Y-%m-%d").ok()?;
    Some((start, end))
}

fn file_icon(is_dir: bool, name: &str) -> &'static str {
    if is_dir {
        "📁"
    } else if name.ends_with(".rs") {
        "🦀"
    } else if name.ends_with(".md") {
        "📄"
    } else if name.ends_with(".toml") {
        "⚙️"
    } else {
        "📦" // default file icon
    }
}

fn main() {
    let args = Args::parse();
    let path = args.path.unwrap_or_else(|| ".".to_string());

    let min_size = args.min_size.as_deref().and_then(parse_size);
    let max_size = args.max_size.as_deref().and_then(parse_size);

    let mod_range = args.modified.as_deref().and_then(parse_date_range);

    let include_pattern = args
        .include
        .as_deref()
        .and_then(|p| Pattern::new(p).ok());
    let exclude_pattern = args
        .exclude
        .as_deref()
        .and_then(|p| Pattern::new(p).ok());

    let mut results = Vec::new();

    let walker = if let Some(depth) = args.depth {
        WalkDir::new(&path).max_depth(depth)
    } else {
        WalkDir::new(&path)
    };

    for entry in walker.into_iter().filter_map(Result::ok) {
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();

        let is_hidden = file_name.starts_with('.');
        if !args.hidden && is_hidden {
            continue;
        }

        // Extension filter
        if let Some(ref ext) = args.ext {
            if file_path.extension().and_then(|e| e.to_str()).map(|e| e != ext).unwrap_or(true) {
                continue;
            }
        }

        // Size filters
        if let Some(min) = min_size {
            if metadata.len() < min {
                continue;
            }
        }
        if let Some(max) = max_size {
            if metadata.len() > max {
                continue;
            }
        }

        // Modified date range filter
        if let Some((start, end)) = mod_range {
            if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Utc> = modified.into();
                let naive = datetime.naive_utc().date();
                if naive < start || naive > end {
                    continue;
                }
            }
        }

        // Pattern include/exclude
        if let Some(ref pattern) = include_pattern {
            if !pattern.matches(&file_name) {
                continue;
            }
        }
        if let Some(ref pattern) = exclude_pattern {
            if pattern.matches(&file_name) {
                continue;
            }
        }

        // Permissions
        let perms = metadata.permissions();
        let perm_str = if perms.readonly() { "r--" } else { "rw-" };

        let info = FileInfo {
            path: file_path.display().to_string(),
            size: metadata.len(),
            modified: metadata
                .modified()
                .ok()
                .map(|t| {
                    let dt = DateTime::<Local>::from(t);
                    dt.format("%Y-%m-%d %H:%M:%S").to_string()
                }),
            is_dir: metadata.is_dir(),
            permissions: perm_str.to_string(),
        };

        results.push(info);
    }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
    } else {
        for item in results {
            let file_name = item.path
                .split(std::path::MAIN_SEPARATOR)
                .last()
                .unwrap_or(&item.path);

            let icon = file_icon(item.is_dir, file_name);

            let name_display = if item.is_dir {
                item.path.blue().bold()
            } else {
                item.path.green()
            };
            
            println!(
                "{} {}  {} bytes  {}   {}",
                icon,
                name_display,
                item.size,
                item.permissions,
                item.modified
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string())
            );
        }
    }
}
