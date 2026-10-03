# fzfx
A fast, semantic CLI tool to search for commands, powered by local embeddings.
tags

media

---

## Features
- **Local Semantic Search:** Uses ONNX-powered local embeddings (`AllMiniLML6V2`) via FastEmbed.
- **Instant Cache:** Smart hit/miss disk caching. Repeated commands bypass model execution entirely.
- **Direct Execution (`-x` / `--exec`):** Automatically runs your chosen command in a subshell once selected.
- **Clipboard Copy (`-c` / `--copy`):** Copies the selected command directly to your system clipboard.
- **Script & Agent Friendly (`-r` / `--raw`):** Outputs only the top match string without interactive terminal UI or extra logs.

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


---

## Examples

### Search and Copy Command
```sh
fzfx -f ~/.bash_history -q "list docker containers" -c
```

### Search and Execute Immediately
```sh
fzfx -f ~/.bash_history -q "find large files" -x
```

### Non-Interactive Script Integration
```sh
CMD=$(fzfx -f commands.txt -q "restart nginx" -r)
eval "$CMD"
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
