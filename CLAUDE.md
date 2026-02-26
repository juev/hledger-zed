# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Zed editor extension for hledger journal files. Provides syntax highlighting via tree-sitter-ledger and LSP support via hledger-lsp.

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

**Language configuration:** `languages/hledger/`
- `config.toml` - file associations (`.journal`, `.hledger`, `.ledger`)
- `highlights.scm` - tree-sitter syntax highlighting queries
- `indents.scm` - auto-indentation rules
- `brackets.scm` - bracket matching
- `outline.scm` - document outline for transactions

**Extension manifest:** `extension.toml` - declares language server config and grammar source (tree-sitter-ledger)

## Testing Locally

1. Remove `grammars/` directory from the repository root (`rm -rf grammars`) — Zed will not install the dev extension if it exists
2. Build the extension
3. Zed → Extensions → Install Dev Extension → select this directory
4. Open a `.journal` file
