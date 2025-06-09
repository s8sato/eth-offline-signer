# eth-offline-signer

[![Static Analysis](https://github.com/s8sato/eth-offline-signer/actions/workflows/static-analysis.yml/badge.svg)](https://github.com/s8sato/eth-offline-signer/actions/workflows/static-analysis.yml)
[![Unit Tests & Coverage](https://github.com/s8sato/eth-offline-signer/actions/workflows/unit-tests.yml/badge.svg)](https://github.com/s8sato/eth-offline-signer/actions/workflows/unit-tests.yml)
[![codecov](https://codecov.io/github/s8sato/eth-offline-signer/graph/badge.svg?token=DGZ6LEM5RC)](https://codecov.io/github/s8sato/eth-offline-signer)
[![Integration Tests](https://github.com/s8sato/eth-offline-signer/actions/workflows/integration-tests.yml/badge.svg)](https://github.com/s8sato/eth-offline-signer/actions/workflows/integration-tests.yml)
[![Sepolia Send](https://github.com/s8sato/eth-offline-signer/actions/workflows/testnet-send.yml/badge.svg)](https://github.com/s8sato/eth-offline-signer/actions/workflows/testnet-send.yml)

A Rust CLI tool for offline signing and JSON-RPC submission of Ethereum-compatible transactions, built on the Alloy library.

## 🔧 Features

- **Offline Signing**
  Generate a fully signed raw transaction (EIP-2718 envelope, hex-encoded) without any network calls.
- **Flexible Fee Models**
  Support for both **EIP-1559** (`max_fee_per_gas` + `max_priority_fee_per_gas`) and **Legacy** (`gas_price`) modes.
- **Configurable**
  Override settings via CLI flags or by using environment variables (via `.env`).
- **RPC Submission**
  Broadcast your signed transaction to any JSON-RPC endpoint and obtain the transaction hash.
- **Confirmation**
  Wait for a transaction to be mined and retrieve its receipt.
- **Auto-generated Documentation**
  CLI help in `docs/cli.md` is maintained automatically via CI.
- **Comprehensive Testing**
  Unit tests, CLI integration tests on Anvil, and optional Sepolia testnet sends.
- **CI Workflows**
  Static analysis, unit tests & coverage, integration tests, CLI-help checks, and manual testnet dispatch—all in GitHub Actions.

## 🛠 Dependencies

- **alloy** for signing, RLP, and JSON-RPC
- **clap v4** for command-line parsing
- **tokio** for async runtime
- **dotenv** for loading `.env` files
- **thiserror**, **color-eyre**, **displaydoc** for ergonomic error handling
- **hex** for hex encoding/decoding

## 📁 Repository Layout

```plain
eth-offline-signer/                   ← root (package.name = "eth-offline-signer")
├── .github/workflows/
│   ├── static-analysis.yml           ← fmt, clippy, audit, CLI-help check
│   ├── unit-tests.yml                ← cargo test & coverage
│   ├── integration-tests.yml         ← Anvil-based integration
│   └── testnet-send.yml              ← Manual Sepolia send workflow
├── docs/
│   └── cli.md                        ← Generated CLI help (clap-markdown)
├── src/
│   ├── main.rs                       ← `eth-offline-signer` binary
│   ├── lib.rs                        ← Library exports and smoke tests
│   ├── sign.rs                       ← Offline signing utilities
│   ├── submit.rs                     ← JSON-RPC submission utilities
│   └── confirm.rs                    ← Transaction confirmation
├── tests/
│   └── cli_integration.rs            ← Anvil CLI integration tests
├── .env.example                      ← Example env variables (RPC_URL, PRIVATE_KEY)
├── CHANGELOG.md                      ← Release notes (Unreleased + tagged)
└── README.md                         ← This document
```

## 🚀 Installation

```bash
git clone https://github.com/s8sato/eth-offline-signer.git
cd eth-offline-signer

cp .env.example .env
# Edit `.env` to set RPC_URL and PRIVATE_KEY

cargo build --release
# Binary available at ./target/release/eth-offline-signer
```

## ⚙️ Usage

See [CLI help](docs/cli.md) for details.

<!-- Sign -->
### 1. Offline Signing

1. **Disconnect** your network (e.g. `nmcli networking off`).
2. Run the `sign` command (no RPC calls):

   ```bash
   ./target/release/eth-offline-signer sign \
     --private-key 0xYOUR_PRIVATE_KEY \
     --chain-id 11155111 \
     --nonce 0 \
     --gas-limit 21000 \
     --to 0xRECIPIENT_ADDRESS \
     --eth 0.001 \
     [eip1559 --max-fee-per-gas 20000000000 --max-priority-fee-per-gas 1000000000] \
     [legacy --gas-price 20000000000]
   ```

   Alternatively, set `PRIVATE_KEY` in your `.env` and omit `--private-key`.


3. **Reconnect** your network (e.g. `nmcli networking on`).

<!-- Submit -->
### 2. RPC Submission

Submit your signed transaction:

```bash
./target/release/eth-offline-signer submit \
  [eip1559 --signed-hex 02GENERATED_RAW_TX] \
  [legacy --signed-hex f8GENERATED_RAW_TX] \
  --rpc-url https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
```

Alternatively, set `RPC_URL` in your `.env` and omit `--rpc-url`.

<!-- Confirm -->
### 3. Confirmation

Wait for mining and fetch the receipt:

```bash
./target/release/eth-offline-signer confirm \
  --tx-hash 0xYOUR_TX_HASH \
  --rpc-url https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
```

Receipt includes: status, block number, gas used, and logs.

## 🌐 Testnet Workflow (Sepolia)

1. **Get Sepolia ETH** from a faucet.
2. **Fetch nonce**:

   ```bash
   curl -s -X POST -H "Content-Type: application/json" \
     --data '{"jsonrpc":"2.0","id":1,"method":"eth_getTransactionCount","params":["0xYOUR_ADDR","latest"]}' \
     $RPC_URL
   ```

3. **Offline sign** as shown above.
4. **Submit** with `submit` command.
5. **Verify** on Sepolia Etherscan:

   ```text
   https://sepolia.etherscan.io/tx/0xYourTxHash
   ```

## 🔧 Configuration

- `.env.example` lists:

  ```text
  RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY
  PRIVATE_KEY=0xYOUR_PRIVATE_KEY
  ```

- CLI flags take precedence over environment variables.

## ✅ Testing

- **Unit tests**:  `cargo test`
- **Integration tests** (Anvil):  via `integration-tests.yml` in CI
- **Optional Sepolia send**:  via `testnet-send.yml` (workflow_dispatch)

## 🤝 Contributing

1. Fork and branch from `main`.
2. Follow [Conventional Commits](https://www.conventionalcommits.org/).
3. Run:

   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

4. Open a pull request.

## 📜 License

MIT © Shunkichi Sato
