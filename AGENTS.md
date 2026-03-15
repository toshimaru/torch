# AGENTS.md

## Project Structure

Entrypoint and tests: [src/main.rs](src/main.rs). Package metadata and dependencies: [Cargo.toml](Cargo.toml). CI: [.github/workflows](.github/workflows). Do not edit `target/` or generated `target/distrib` outputs manually.

## Commands

- **Build:** `cargo build`
- **Run:** `cargo run -- <paths...>`
- **Test (all):** `cargo test`
- **Test (single):** `cargo test <test_name>`
- **Lint:** `cargo clippy`
- **Format:** `cargo fmt` / check with `cargo fmt --check`

## Architecture

Single-file CLI tool (`src/main.rs`) that combines `mkdir -p` and `touch`. Given a path like `a/b/c.txt`, it creates intermediate directories then creates/touches the file.

Core functions: `mkdir_touch` (orchestrator), `mkdir` (creates directories via `create_dir_all`), `touch` (creates file or updates timestamps).

Dependencies: `clap` (CLI parsing with derive), `filetime` (timestamp manipulation).

## Release & Distribution

Uses [cargo-dist](https://github.com/axodotdev/cargo-dist) for releases. Config in `dist-workspace.toml`.

- Targets: macOS (aarch64, x86_64), Linux (aarch64, x86_64, musl), Windows (x86_64)
- Installers: shell script, Homebrew ([toshimaru/homebrew-torch](https://github.com/toshimaru/homebrew-torch))
- CI: GitHub Actions, runs plan on PRs
