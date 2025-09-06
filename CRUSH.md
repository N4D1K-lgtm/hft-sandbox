# CRUSH.md - HFT Sandbox Development Guide

## Build/Test/Lint Commands

- **Build**: `cargo build` (debug) / `cargo build --release` (optimized)
- **Run**: `cargo run` (debug) / `cargo run --release` (optimized)
- **Test all**: `cargo test`
- **Test single**: `cargo test <test_name>` (partial name matching)
- **Lint**: `cargo clippy` (warnings) / `cargo clippy -- -D warnings` (fail on warnings)
- **Format**: `cargo fmt` (format all files) / `cargo fmt --check` (check only)
- **Clean**: `cargo clean`

## Code Style Guidelines

- **Performance Priority**: This is an HFT system - optimize for latency and throughput
- **Rust Edition**: 2024 (latest features encouraged)
- **Formatting**: Use `cargo fmt` with default rustfmt settings
- **Linting**: Follow all clippy suggestions, especially performance-related ones
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE_CASE for constants
- **Imports**: Group std, external crates, then local modules with blank lines between
- **Error Handling**: Use `Result<T, E>` for fallible operations, avoid `unwrap()` in production code
- **Types**: Prefer explicit types for public APIs, use type inference for local variables
- **Memory**: Minimize allocations, prefer stack allocation, use `&str` over `String` when possible
- **Concurrency**: Use async/await for I/O, consider lock-free data structures for hot paths
- **Documentation**: Document public APIs with `///`, include examples for complex functions
- **Testing**: Unit tests in same file with `#[cfg(test)]`, integration tests in `tests/` directory
- **Dependencies**: Minimize external dependencies, prefer no_std compatible crates when possible

## Project Context

Educational HFT system focusing on performance optimization and low-latency algorithms.

