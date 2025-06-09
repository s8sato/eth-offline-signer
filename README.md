# eth-offline-signer

A Rust-based CLI for offline signing and JSON-RPC submission of Ethereum-compatible transactions, powered by Alloy.

## Features

- **Offline signing**
  Generate a fully signed raw transaction (EIP-2728-encoded hex) without any network calls.
- **EIP-1559 and Legacy fee models**
  Choose the modern `max_fee_per_gas` + `max_priority_fee_per_gas` model or the classic `gas_price` model.
- **Configurable**
  Override settings via CLI flags, or default to environment variables (using `.env`).
- **RPC submission**
  Submit your signed raw transaction to any JSON-RPC endpoint and retrieve the transaction hash.
- **CLI documentation**
  Auto-generated help in `docs/cli.md`, kept in sync via CI.
- **Comprehensive testing**
  Unit tests, property-based tests (Anvil + proptest), and optional Goerli testnet submissions.
- **CI workflows**
  Static analysis, unit tests & coverage, integration tests, and CLI-help consistency checks—all automated in GitHub Actions.

## Dependencies

- `alloy` for managing entire transaction lifecycle
- `clap` v4 for CLI parsing
- `tokio` for async runtime
- `dotenv` for `.env` config
- `thiserror` + `color-eyre` + `displaydoc` for error handling
- `hex` for hex encoding/decoding
- `proptest` for property tests
- `serde_json` for JSON utilities

## Repository Layout

```plain
eth-offline-signer/                   ← root (package.name = "eth-offline-signer")
├── .github/workflows/
│   ├── static-analysis.yml           ← fmt, clippy, cargo-audit, CLI help 
│   ├── unit-tests.yml                ← cargo test, coverage
│   └── integration-tests.yml         ← Anvil + testnet runs
├── docs/
│   └── cli.md                        ← auto-generated CLI help (clap-markdown)
├── src/
│   ├── main.rs                       ← binary entry point
│   ├── lib.rs                        ← library exports and smoke tests
│   ├── sign.rs                       ← offline-signing utilities
│   ├── submit.rs                     ← JSON-RPC submission utilities
│   └── confirm.rs                    ← Transaction confirmation
├── tests/
│   └── cli_integration.rs            ← Anvil submission tests using CLI
├── .env.example                      ← template for RPC_URL, PRIVATE_KEY
├── CHANGELOG.md                      ← release notes (Unreleased + tags)
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

### 1. Offline signing

1. **Disconnect** your network (e.g. `nmcli networking off`).

2. Run the `sign` command:

   ```bash
   ./target/release/eth-offline-signer sign \
     --private-key 0xYOUR_PRIVATE_KEY \
     --chain-id 5 \
     --nonce 5 \
     --gas-limit 21000 \
     --to 0xRECIPIENT_ADDRESS \
     --eth 0.01 \
     [eip1559 --max-fee-per-gas 50 --max-priority-fee-per-gas 2] \
     [legacy --gas-price 10]
   ```

   See [CLI help](docs/cli.md#eth-offline-signer-sign) for details.

3. **Reconnect** your network (e.g. `nmcli networking on`).

### 2. RPC submission

```bash
./target/release/eth-offline-signer submit eip1559 \
  --signed-hex 02GENERATED_RAW_TX \
  --rpc-url https://eth-goerli.alchemyapi.io/v2/YOUR_KEY
```

Or set `RPC_URL` in your `.env` and omit `--rpc-url`.

### 3. Confirmation

After submission, wait for the transaction to be mined and retrieve its receipt:

```bash
./target/release/eth-offline-signer confirm \
  --tx-hash 0xYOUR_TX_HASH \
  --rpc-url https://eth-goerli.alchemyapi.io/v2/YOUR_KEY
```

Once the receipt is available, it will be printed including status, block number, gas used, and any logs.

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
