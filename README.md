# hledger-zed

Zed extension for [hledger](https://hledger.org/) journal files with LSP integration.

## Features

- LSP support via [hledger-lsp](https://github.com/juev/hledger-lsp):
  - Syntax highlighting via semantic tokens
  - Autocompletion for accounts and commodities
  - Hover information
  - Diagnostics for unbalanced transactions
  - Go to definition
  - Document formatting
- Support for `.journal`, `.hledger`, `.ledger`, and `.rules` files

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
            "diagnostics": true,
            "semanticTokens": true,
            "codeActions": true,
            "foldingRanges": true,
            "documentLinks": true,
            "workspaceSymbol": true,
            "inlineCompletion": true,
            "codeLens": false
          },
          "completion": {
            "maxResults": 50,
            "fuzzyMatching": true,
            "showCounts": true,
            "includeNotes": true
          },
          "diagnostics": {
            "undeclaredAccounts": true,
            "undeclaredCommodities": true,
            "unbalancedTransactions": true,
            "balanceTolerance": 0.0
          },
          "formatting": {
            "indentSize": 4,
            "alignAmounts": true,
            "minAlignmentColumn": 40
          },
          "cli": {
            "enabled": true,
            "path": "hledger",
            "timeout": 30000
          },
          "limits": {
            "maxFileSizeBytes": 10485760,
            "maxIncludeDepth": 50
          }
        }
      }
    }
  }
}
```

### Settings Reference

#### Features

| Setting | Default | Description |
|---------|---------|-------------|
| `features.hover` | `true` | Hover information |
| `features.completion` | `true` | Completions |
| `features.formatting` | `true` | Document formatting |
| `features.diagnostics` | `true` | Diagnostics |
| `features.semanticTokens` | `true` | Semantic tokens |
| `features.codeActions` | `true` | Code actions |
| `features.foldingRanges` | `true` | Folding ranges for transactions and directives |
| `features.documentLinks` | `true` | Clickable links for include directives |
| `features.workspaceSymbol` | `true` | Workspace symbol search |
| `features.inlineCompletion` | `true` | Ghost text completions for transaction templates |
| `features.codeLens` | `false` | Balance check indicators on transactions |

#### Completion

| Setting | Default | Description |
|---------|---------|-------------|
| `completion.maxResults` | `50` | Maximum number of completion items |
| `completion.fuzzyMatching` | `true` | Enable fuzzy matching |
| `completion.showCounts` | `true` | Show usage counts in completion details |
| `completion.includeNotes` | `true` | Include notes in payee completions |

#### Diagnostics

| Setting | Default | Description |
|---------|---------|-------------|
| `diagnostics.undeclaredAccounts` | `true` | Report undeclared accounts |
| `diagnostics.undeclaredCommodities` | `true` | Report undeclared commodities |
| `diagnostics.unbalancedTransactions` | `true` | Report unbalanced transactions |
| `diagnostics.balanceTolerance` | `0.0` | Tolerance for balance checks (0 = exact) |

#### Formatting

| Setting | Default | Description |
|---------|---------|-------------|
| `formatting.indentSize` | `4` | Number of spaces for posting indent |
| `formatting.alignAmounts` | `true` | Align amounts across postings |
| `formatting.minAlignmentColumn` | `40` | Minimum column for amount alignment (0 = auto) |

#### CLI

| Setting | Default | Description |
|---------|---------|-------------|
| `cli.enabled` | `true` | Enable hledger CLI integration |
| `cli.path` | `"hledger"` | Path to hledger executable |
| `cli.timeout` | `30000` | CLI command timeout in milliseconds |

#### Limits

| Setting | Default | Description |
|---------|---------|-------------|
| `limits.maxFileSizeBytes` | `10485760` | Maximum journal file size (bytes) |
| `limits.maxIncludeDepth` | `50` | Maximum include depth for recursive loading |

## Semantic Tokens

Syntax highlighting is provided entirely by hledger-lsp semantic tokens. hledger-lsp uses standard LSP token types, so highlighting works out of the box with any Zed theme.

Zed does not enable semantic tokens by default — add the following to your `settings.json`:

```json
{
  "languages": {
    "hledger": {
      "semantic_tokens": "full"
    },
    "hledger-rules": {
      "semantic_tokens": "full"
    }
  }
}
```

## Supported File Extensions

- `.journal`
- `.hledger`
- `.ledger`
- `.rules` (CSV import rules)

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
