# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building
- **Build Solana program**: `cargo build-sbf`
- **Build all workspace members**: `cargo build`
- **Build with release optimizations**: `cargo build-sbf --release`

### Testing
- **Quick test**: `./test.sh` (builds program and runs tests)
- **Manual testing**:
  1. Build program first: `cargo build-sbf`
  2. Run tests: `SBF_OUT_DIR="$(pwd)/target/deploy" cargo test`
- **Run specific test**: `SBF_OUT_DIR="$(pwd)/target/deploy" cargo test test_name`
- **Run tests with output**: `SBF_OUT_DIR="$(pwd)/target/deploy" cargo test -- --nocapture`

**Note**: Mollusk requires the `SBF_OUT_DIR` environment variable to locate the compiled program.

### IDL and Client Generation
- **Generate IDL**: `shank idl -r program -o idl` or `yarn generate-idl`
- **Generate clients**: `yarn generate-clients` (requires IDL to be generated first)

### Linting and Formatting
- **Format code**: `cargo fmt`
- **Check formatting**: `cargo fmt -- --check`
- **Run clippy**: `cargo clippy`

## Architecture Overview

This is a Solana program template using the **Pinocchio** framework, which is optimized for performance and minimal CU usage.

### Key Components

1. **Program Structure** (`program/`):
   - **Entry point**: `src/entrypoint.rs` - Routes instructions based on discriminator
   - **Processors**: `src/processor/` - Contains business logic for each instruction
     - `initialize.rs` - Initializes counter accounts
     - `increment.rs` - Increments counter value
   - **State**: `src/state/` - Defines on-chain account structures using Shank attributes
   - **Errors**: `src/error.rs` - Custom program errors with Shank support

2. **Testing Framework**:
   - Uses **Mollusk** (formerly LiteSVM) for integration tests
   - Tests located in `tests/src/lib.rs`
   - Simulates Solana runtime locally for fast testing

3. **Client Generation**:
   - Uses **Codama** for client generation from Shank IDL
   - Generated clients in `clients/rust/src/generated/`
   - Auto-generated from IDL, do not edit manually

4. **Instruction Routing**:
   - Instructions identified by single-byte discriminator
   - 0 = Initialize
   - 1 = Increment

### Important Notes

- The program ID is currently a placeholder (`2NCpU9nsgLfhqKX5CDVz24FfsZ8aRwjgUWtFbPsVZVu2`) and needs updating in `program/src/lib.rs`
- This template uses `#![no_std]` for optimal performance
- Pinocchio provides zero-copy deserialization and minimal overhead
- Shank attributes are used for IDL generation and client code generation