# hledger-zed

Zed extension for [hledger](https://hledger.org/) journal files with LSP integration.

## Features

- Syntax highlighting via [tree-sitter-hledger](https://github.com/chrislloyd/tree-sitter-hledger)
- LSP support via [hledger-lsp](https://github.com/juev/hledger-lsp):
  - Autocompletion for accounts and commodities
  - Hover information
  - Diagnostics for unbalanced transactions
  - Go to definition
  - Document formatting

## Installation

### From Zed Extensions

1. Open Zed
2. Go to Extensions (Cmd+Shift+X on macOS)
3. Search for "hledger"
4. Click Install

### Manual Installation

1. Clone this repository
2. Open Zed and go to Extensions
3. Click "Install Dev Extension" and select the cloned directory

## LSP Binary

The extension automatically downloads `hledger-lsp` from [GitHub Releases](https://github.com/juev/hledger-lsp/releases) when needed.

If you prefer to install it manually:

```bash
# Using Go
go install github.com/juev/hledger-lsp@latest

# Or download from releases
# https://github.com/juev/hledger-lsp/releases
```

## Configuration

Configure the LSP in your Zed `settings.json`:

```json
{
  "lsp": {
    "hledger-lsp": {
      "binary": {
        "path": "/custom/path/to/hledger-lsp"
      },
      "initialization_options": {
        "hledger": {
          "features": {
            "hover": true,
            "completion": true,
            "formatting": true,
            "diagnostics": true
          },
          "completion": {
            "maxResults": 50,
            "snippets": true,
            "fuzzyMatching": true
          },
          "diagnostics": {
            "undeclaredAccounts": true,
            "undeclaredCommodities": true,
            "unbalancedTransactions": true
          },
          "formatting": {
            "indentSize": 4,
            "alignAmounts": true
          }
        }
      }
    }
  }
}
```

## Supported File Extensions

- `.journal`
- `.hledger`
- `.ledger`

## Development

### Prerequisites

- [Rust](https://rustup.rs/)
- wasm32-wasip1 target: `rustup target add wasm32-wasip1`

### Building

```bash
cargo build --target wasm32-wasip1 --release
```

### Testing Locally

1. Build the extension
2. Open Zed → Extensions → Install Dev Extension
3. Select this directory
4. Open a `.journal` file to test

## License

MIT
