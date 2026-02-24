# AGENTS.md

## Cursor Cloud specific instructions

This is a multi-language coding challenge repository (Python, Go, Rust) with no running services — only standalone exercises and tests.

### Languages & Versions

| Language | Version | Manager | Config |
|----------|---------|---------|--------|
| Python | ^3.9 | Poetry | `python/pyproject.toml` |
| Go | 1.24.5 | Go modules | `golang/go.mod` |
| Rust | edition 2021+ (some use 2024) | Cargo | per-project `Cargo.toml` |

### Running Tests

- **Python**: `cd python && poetry run pytest` (113 tests)
- **Go**: `cd golang && go test ./...` (only Week-01 Day-01 and Day-02 have test files)
- **Rust**: `cd rust/<challenge-dir> && cargo test` (each challenge is a standalone Cargo project)

### Linting

- **Python**: `cd python && poetry run python -m pylint <file>` — pylint must be invoked with module-style paths or from the correct cwd
- **Go**: `cd golang && go vet ./...`
- **Rust**: `cd rust/<challenge-dir> && cargo clippy`

### Gotchas

- The Rust `Week-14/Day-98_Geography-Of-Brazil` project depends on `openssl-sys` → requires `libssl-dev` and `pkg-config` system packages.
- The Rust `Week-01/Day-02_Finding-Nemo/finding_nemo/Cargo.toml` is missing a `[package]` section — this is a repo issue, not an environment problem. Skip it during bulk test runs.
- Python `pyproject.toml` does not set `package-mode = false`, so `poetry install` without `--no-root` will warn. Use `poetry install --no-root` to suppress.
- Go 1.24.5 is required by `go.mod`; the system default Go (1.22) is too old — Go must be installed to `/usr/local/go`.
- Rust stable must support edition 2024 for the geography challenge. Run `rustup default stable` if the default is pinned to an older version.
