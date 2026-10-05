# fzfx

[![Crates.io](https://img.shields.io/crates/v/fzfx?style=for-the-badge&logo=rust&color=e05d44)](https://crates.io/crates/fzfx)
[![GitHub Release](https://img.shields.io/github/v/release/aether-flux/fzfx?style=for-the-badge&logo=github&color=2bbc8a)](https://github.com/aether-flux/fzfx/releases/latest)
[![License](https://img.shields.io/github/license/aether-flux/fzfx?style=for-the-badge&color=8a2be2)](LICENSE)

[![asciicast](https://asciinema.org/a/mpEu3J0KEHLaUZVo)](https://asciinema.org/a/mpEu3J0KEHLaUZVo)

`fzfx` is an offline, fast, and semantic CLI command finder designed to instantly discover previously executed terminal commands using vector search rather than rigid exact or fuzzy substring matching.

---

## Features
- **Local Semantic Search:** Uses ONNX-powered local embeddings (`AllMiniLML6V2`) via FastEmbed.
- **Instant Cache:** Smart hit/miss disk caching. Repeated commands bypass model execution entirely.
- **Direct Execution (`-x` / `--exec`):** Automatically runs your chosen command in a subshell once selected.
- **Clipboard Copy (`-c` / `--copy`):** Copies the selected command directly to your system clipboard.
- **Script & Agent Friendly (`-r` / `--raw`):** Outputs only the top match string without interactive terminal UI or extra logs.
- **Clear Cache**: Use `--clear-cache` or `-C` to clear embeddings cache, and `--clear-all` to clear both model and embedding cache files.

---

## Installation

### Quick Install (Shell Script)
```sh
curl -fsSL https://raw.githubusercontent.com/aether-flux/fzfx/main/install.sh | sh
```

### Cargo Install
```sh
cargo install fzfx
```

### Manual Build
```sh
# Clone the repo
git clone https://github.com/aether-flux/fzfx
cd fzfx

# Run the CLI
cargo run    # with additional flags and arguments
```

---

## Usage

### Basic Interactive Search
Pipe any command list or history file into `fzfx`:
```sh
# Search through bash history
cat ~/.bash_history | fzfx -q "check disk space"

# Search through your terminal's history
history | fzfx -q "list disk partitions"

# Search active process list
ps aux | fzfx -q "kill web server"
```

If no query (`-q` or `--query`) is supplied, `fzfx` prompts for input interactively:
```sh
fzfx -f ~/.bash_history
```

---

## Flags and Options

| Flag | Long Flag            | Description                                                   |
| ---- | -------------------- | ------------------------------------------------------------- |
| `-q` | `--query <TEXT>`     | Initial search query                                          |
| `-f` | `--data-file <PATH>` | Path to candidate file (one entry per line)                   |
| `-x` | `--exec`             | Automatically execute selected command                        |
| `-c` | `--copy`             | Copy selected command to clipboard                            |
| `-r` | `--raw`              | Print top match only (non-interactive, for scripts/AI agents) |
| `-k` | `--top-k <NUM>`      | Specify number of results to show (default = 10)              |
| `-C` | `--clear-cache`      | Remove cache files for command embedding                      |
| NA   | `--clear-all`        | Remove both model and embedding cache files                   |


---

## Examples

### Search and Copy Command
```sh
fzfx -f ~/.bash_history -q "list docker containers" -c
```

### Search and Execute Immediately
```sh
history | fzfx -q "find large files" -x
```

### Non-Interactive Script Integration
```sh
CMD=$(fzfx -f commands.txt -q "restart nginx" -r)
eval "$CMD"
```

### Clear all cache files
```sh
fzfx --clear-all
```

---

## Shell Integration
Add a convenient shortcut to your `~/.zshrc` or `~/.bashrc` to invoke `fzfx` directly over your command history:
```sh
fz() {
    local cmd
    cmd=$(cat ~/.bash_history | fzfx -q "$*" -c)
}
```

## Cache and Storage
`fzfx` respects standard OS cache paths:
- Linux: `~/.cache/fzfx/`
- macOS: `~/Library/Caches/fzfx/`
- Windows: `%LOCALAPPDATA%\fzfx\cache\`

Model weights are cached under their specific directories and precomputed line vectors are serialized into `embeddings.bin`. To reset your search index cache, simply delete `embeddings.bin`.

---

## License
MIT
