# eth-offline-signer

A Rust-based CLI for offline signing and JSON-RPC submission of Ethereum-compatible transactions, powered by Alloy.

## Features

- **Offline signing**
  Generate a fully signed raw transaction (RLP-serialized hex) without any network calls.
- **EIP-1559 & Legacy fee models support**
  Choose the modern `max_fee_per_gas` + `max_priority_fee_per_gas` (default) or the classic `gas_price` models.
- **Configurable**
  Override settings via CLI flags, or environment variables (using `.env` file).
- **RPC submission**
  Submit your signed raw transaction to any JSON-RPC endpoint and retrieve the transaction hash.
- **Comprehensive testing**
  Unit tests, property-based tests (Anvil + proptest), and optional Goerli testnet submissions.
- **CI workflows**
  Static analysis, unit tests & coverage, integration tests—all automated in GitHub Actions.

## Dependencies

- `alloy` for signing, RLP, and JSON-RPC
- `clap` v4 for CLI parsing
- `tokio` for async runtime
- `dotenv` for `.env` config
- `thiserror` + `color-eyre` for error handling
- `hex` for hex encoding/decoding
- `proptest` for property tests
- `serde_json` for JSON utilities

## Repository Layout

```plain

eth-offline-signer/                    ← root (package.name = "eth-offline-signer")
├── .github/workflows/
│   ├── static-analysis.yml           ← fmt, clippy, cargo-audit
│   ├── unit-tests.yml                ← cargo test, coverage
│   └── integration-tests.yml         ← Anvil + testnet runs
├── src/
│   ├── main.rs                       ← binary entry point
│   ├── lib.rs                        ← library exports
│   ├── offline_sign.rs               ← offline-signing utilities
│   ├── rpc_submit.rs                 ← JSON-RPC submission utilities
│   └── utils/
│       ├── mod.rs
│       └── conversions.rs            ← Ether⇄Wei, hex conversions
├── tests/
│   ├── sign_integration.rs           ← known-vector signing tests
│   └── submit_proptest.rs            ← proptest + Anvil submission tests
├── .env.example                      ← template for RPC_URL, PRIVATE_KEY
├── .gitignore
├── Cargo.toml
├── CHANGELOG.md                      ← release notes (Unreleased + tags)
├── LICENSE                           ← MIT license
├── clippy.toml                       ← Clippy configuration
├── rustfmt.toml                      ← rustfmt configuration
├── docs/
│   └── cli.md                        ← auto-generated CLI help (clap-markdown)
└── README.md                         ← this document

```

## Installation

```bash
git clone https://github.com/yourusername/eth-offline-signer.git
cd eth-offline-signer

cp .env.example .env
# Edit `.env` to set RPC_URL and PRIVATE_KEY

cargo build --release
# Binary: ./target/release/eth-offline-signer
```

## Usage

### 1. Offline Signing

1. **Disconnect** your network (e.g. `nmcli networking off`).

2. Run the `sign` command (no RPC calls):

   ```bash
   ./target/release/eth-offline-signer sign \
     --private-key 0xYOUR_PRIVATE_KEY \
     --to 0xRECIPIENT_ADDRESS \
     --amount 0.01 \
     --nonce 5 \
     --max-fee-per-gas 50 \
     --max-priority-fee-per-gas 2 \
     [--gas-price-gwei 10] \
     --gas-limit 21000 \
     --chain-id 5 \
   ```

   - `--private-key`: 0x-prefixed hex private key
   - `--to`: 0x-prefixed recipient address
   - `--amount`: ETH amount as string (e.g. `"0.01"`)
   - `--nonce`: pre-fetched nonce
   - `--max-fee-per-gas`, `--max-priority-fee-per-gas` (default): EIP-1559 fees in Gwei
   - `--gas-price-gwei` (alternative): legacy gas price in Gwei
   - `--gas-limit`: gas limit (default: 21000)
   - `--chain-id`: chain ID (1=mainnet, 5=Goerli, 56=BSC, etc.)

3. **Reconnect** your network (e.g. `nmcli networking on`).

### 2. RPC Submission

```bash
./target/release/eth-offline-signer submit \
  --raw-tx 0xGENERATED_RAW_TX \
  --rpc-url https://eth-goerli.alchemyapi.io/v2/YOUR_KEY
```

Or set `RPC_URL` in your `.env` and omit `--rpc-url`.

## Testnet Workflow

1. **Obtain test ETH** via Goerli/Sepolia faucets.

2. **Fetch nonce & gas fees** once:

   ```bash
   curl -X POST -H "Content-Type: application/json" \
     --data '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionCount","params":["0xYOUR_ADDR","latest"]}' \
     $RPC_URL
   ```

3. **Offline sign** (see above).

4. **Submit** with `submit` command.

5. **Verify** on Etherscan:

   ```plain
   https://goerli.etherscan.io/tx/0xYourTxHash
   ```

## Configuration

- `.env.example` shows environment variables:

  ```text
  RPC_URL=https://eth-goerli.alchemyapi.io/v2/YOUR_KEY
  PRIVATE_KEY=0xYOUR_PRIVATE_KEY
  ```

- CLI flags override env vars.

## Testing

- **Unit tests**: `cargo test`
- **Property tests** (Anvil + proptest): run via `integration-tests.yml` in CI
- **Optional Goerli tests**: uses GitHub Secrets for PRIVATE_KEY

## Contributing

1. Fork & branch from `main`.
2. Follow [Conventional Commits](https://www.conventionalcommits.org/).
3. Run:

   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

4. Submit a PR.

## License

MIT © Shunkichi Sato
