<div align="center">

# SLS

Sls is a lightning-fast Rust alternative to ls with powerful filtering, colourful output, and intuitive icons.

</div>

## Usage

```bash
sls [OPTIONS] [PATH]
```

PATH - (optional) Directory to scan. Defaults to the current working directory (.) if not provided.

### Options & Flags

| Flag / Option                | Description                                               | Example                           |
|------------------------------|-----------------------------------------------------------|-----------------------------------|
| `--ext <EXT>`                 | Filter by file extension                                  | `--ext rs`                       |
| `--min-size <SIZE>`           | Minimum file size to include. Supports B, KB, MB, GB.     | `--min-size 10KB`                |
| `--max-size <SIZE>`           | Maximum file size to include. Supports B, KB, MB, GB.     | `--max-size 1MB`                 |
| `--hidden`                    | Include hidden files (dotfiles)                           | `--hidden`                       |
| `--modified <START..END>`     | Filter by modification date range (YYYY-MM-DD..YYYY-MM-DD) | `--modified 2025-01-01..2025-08-31` |
| `--include <PATTERN>`         | Include only files matching a glob pattern               | `--include "*.rs"`               |
| `--exclude <PATTERN>`         | Exclude files matching a glob pattern                    | `--exclude "*test*"`             |
| `--json`                      | Output in JSON format instead of pretty colours/icons     | `--json`                         |
| `--depth <N>`                 | Limit recursive scan to N directory levels               | `--depth 2`                      |
| `-h, --help`                  | Show help message                                         | `--help`                         |
| `-V, --version`               | Show version number                                       | `--version`                      |

### Example Commands

```bash
# List all files in current directory
sls

# List only Rust source files larger than 1 KB
sls --ext rs --min-size 1KB

# List Markdown files, excluding drafts, from Jan to Aug 2025
sls --ext md --exclude "*draft*" --modified 2025-01-01..2025-08-31

# Output results in JSON format
sls --json

# Limit recursion to depth 2
sls --depth 2
```