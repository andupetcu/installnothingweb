# Repository Guidelines

## Project Structure & Modules
- `src/main.rs`: entrypoint; wires CLI and installer.
- `src/cli.rs`: CLI flags and `Stage` enum; includes unit tests.
- `src/installer.rs`: core loop, header UI, stage orchestration.
- `src/stages/`: each installation stage as a module implementing `InstallationStage` via `src/stages/mod.rs`.
- `src/ui/`: terminal spinner/progress components.
- `src/*_logs.rs` and `data/*.log`: sample/demo log content.

Tip: When adding a stage, create `src/stages/<name>.rs`, expose it in `src/stages/mod.rs`, and add a variant in `cli::Stage` plus matching construction in `selected_stages`.

## Build, Test, and Dev
- Build: `cargo build` (use `--release` for optimized binary).
- Run: `cargo run -- --help` or `cargo run -- --all`.
- Lint: `cargo clippy -- -D warnings` (treat warnings as errors).
- Format: `cargo fmt --all`.
- Test: `cargo test`.

## Coding Style & Naming
- Rust 2021 edition; 4-space indentation; rustfmt-required.
- Types/traits: `CamelCase`; modules/files/functions: `snake_case`.
- Prefer `Result` propagation over `unwrap()`. Avoid `unsafe`.
- Keep modules focused; group UI in `src/ui`, stages in `src/stages`.

## Testing Guidelines
- Unit tests live next to code with `#[cfg(test)] mod tests { ... }` (see `src/cli.rs`).
- For broader coverage, add integration tests under `tests/` using `cargo test`.
- Name tests by behavior: `test_<component>_<behavior>()`.

## Commit & Pull Requests
- Commits: imperative mood, concise, scoped. Optional prefixes like `feat:`, `fix:`, `refactor:` are welcome but not required (existing history is mixed).
- PRs should include:
  - Summary of changes and rationale.
  - Linked issue (if any) and affected stages/modules.
  - Run output or short terminal screenshot/gif when changing UI/flow.
  - Confirmation that `cargo fmt`, `cargo clippy`, and `cargo test` pass.

## Security & Configuration
- Do not introduce network calls or external process execution without discussion.
- Keep logs/demo assets small; prefer embedding or generation utilities when possible.
- Handle terminal state carefully: always restore raw mode on errors.

## Agent-Specific Notes
- Follow this guide for any edits within this repo’s tree.
- Keep changes minimal and localized; do not reformat unrelated files.
- If adding a new stage, update `cli::Stage`, `src/stages/mod.rs`, and provide a sensible default in `SimulationConfig`.
