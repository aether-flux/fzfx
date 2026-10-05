# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-05

### Added
- **Hybrid Search Engine:** Combines semantic vector similarity embeddings with fuzzy string matching.
- **Additional Flags**: Added flags to clear model and embedding cache files, and to control the number of results to display.
- **Benchmark Suite:** Added Criterion benchmarking suite covering model initialization, cold cache, and warm cache hit paths.
- **Cross-Platform Release Builds:** Added pre-compiled binary targets for both `x86_64` and `aarch64` architectures.

### Changed
- Added extra context to certain commands (eg, `rm` - `remove delete files`) for non-obvious intent matching.
- Refactored core modules into a library crate structure (`lib.rs`) for testability and benchmarking.

---

## [0.1.0] - 2026-10-03

### Added
- **Local Semantic Search:** Uses ONNX-powered local embeddings (`AllMiniLML6V2`) via FastEmbed.
- **Instant Cache:** Smart hit/miss disk caching. Repeated commands bypass model execution entirely.
- **Direct Execution (`-x` / `--exec`):** Automatically runs your chosen command in a subshell once selected.
- **Clipboard Copy (`-c` / `--copy`):** Copies the selected command directly to your system clipboard.
- **Script & Agent Friendly (`-r` / `--raw`):** Outputs only the top match string without interactive terminal UI or extra logs.
