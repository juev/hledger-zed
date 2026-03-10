# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Zed editor extension for hledger journal and rules files. Provides syntax highlighting via semantic tokens and LSP support via hledger-lsp.

## Build Commands

```bash
# Build the extension (produces WASM)
cargo build --target wasm32-wasip1 --release

# Check code without building
cargo check --target wasm32-wasip1
```

Prerequisites: Rust with wasm32-wasip1 target (`rustup target add wasm32-wasip1`).

## Architecture

**Extension entry point:** `src/lib.rs` - implements `zed::Extension` trait with:
- `new()` - initializes extension state
- `language_server_command()` - returns command to run hledger-lsp
- Binary resolution: first checks PATH for `hledger-lsp`, then downloads from GitHub releases

**Language configurations:**
- `languages/hledger/config.toml` - journal file associations (`.journal`, `.hledger`, `.ledger`)
- `languages/hledger-rules/config.toml` - rules file associations (`.rules`)

**Extension manifest:** `extension.toml` - declares language server config, hledger-lsp serves both languages

**Syntax highlighting:** Basic highlighting via tree-sitter grammar (`cbarrete/tree-sitter-ledger`). Enhanced highlighting via hledger-lsp semantic tokens (optional, requires `semantic_tokens` setting in `settings.json`).

## Pre-commit Checks

Before every commit, run:

```bash
cargo fmt --check && cargo check --target wasm32-wasip1
```

Fix any issues before committing. Do not skip these checks.

## Testing Locally

1. Build the extension
2. Zed → Extensions → Install Dev Extension → select this directory
3. Open a `.journal` or `.rules` file
