# Command-Line Help for `eth-offline-signer`

This document contains the help content for the `eth-offline-signer` command-line program.

**Command Overview:**

* [`eth-offline-signer`↴](#eth-offline-signer)
* [`eth-offline-signer sign`↴](#eth-offline-signer-sign)
* [`eth-offline-signer sign eip1559`↴](#eth-offline-signer-sign-eip1559)
* [`eth-offline-signer sign legacy`↴](#eth-offline-signer-sign-legacy)
* [`eth-offline-signer submit`↴](#eth-offline-signer-submit)
* [`eth-offline-signer confirm`↴](#eth-offline-signer-confirm)
* [`eth-offline-signer markdown-help`↴](#eth-offline-signer-markdown-help)

## `eth-offline-signer`

CLI for offline signing and RPC submission of Ethereum-compatible transactions

**Usage:** `eth-offline-signer <COMMAND>`

###### **Subcommands:**

* `sign` — Offline-only transaction signing (no network calls)
* `submit` — Submit a previously signed raw transaction via JSON-RPC
* `confirm` — Wait until a transaction is first included in a block and print its receipt
* `markdown-help` — Output CLI documentation in Markdown format



## `eth-offline-signer sign`

Offline-only transaction signing (no network calls)

**Usage:** `eth-offline-signer sign [OPTIONS] --private-key <PRIVATE_KEY> --chain-id <CHAIN_ID> --nonce <NONCE> --to <TO> --eth <eth> <COMMAND>`

###### **Subcommands:**

* `eip1559` — Use the EIP-1559 fee market model
* `legacy` — Use the legacy gas price model

###### **Options:**

* `--private-key <PRIVATE_KEY>` — 0x-prefixed private key for signing
* `--chain-id <CHAIN_ID>` — Chain ID (e.g. 1 for Mainnet, 11155111 for Sepolia)
* `--nonce <NONCE>` — Transaction nonce (pre-fetched from RPC)
* `--gas-limit <GAS_LIMIT>` — Maximum gas units to allow for this transaction

  Default value: `21000`
* `--to <TO>` — 0x-prefixed recipient address
* `--eth <eth>` — Amount to send in ETH (e.g. "0.01")



## `eth-offline-signer sign eip1559`

Use the EIP-1559 fee market model

**Usage:** `eth-offline-signer sign eip1559 --max-fee-per-gas <MAX_FEE_PER_GAS> --max-priority-fee-per-gas <MAX_PRIORITY_FEE_PER_GAS>`

###### **Options:**

* `--max-fee-per-gas <MAX_FEE_PER_GAS>` — Maximum total fee per gas in Wei
* `--max-priority-fee-per-gas <MAX_PRIORITY_FEE_PER_GAS>` — Maximum priority fee per gas (tip) in Wei



## `eth-offline-signer sign legacy`

Use the legacy gas price model

**Usage:** `eth-offline-signer sign legacy --gas-price <GAS_PRICE>`

###### **Options:**

* `--gas-price <GAS_PRICE>` — Gas price in Wei



## `eth-offline-signer submit`

Submit a previously signed raw transaction via JSON-RPC

**Usage:** `eth-offline-signer submit --signed-hex <SIGNED_HEX> --rpc-url <RPC_URL> <TX_TYPE>`

###### **Arguments:**

* `<TX_TYPE>` — Transaction type: EIP-1559 (Type 2) or Legacy (Type 0)

  Possible values:
  - `eip1559`:
    Use the EIP-1559 fee market (Type-2 transaction)
  - `legacy`:
    Use the legacy gas price model (Type-0 transaction)


###### **Options:**

* `--signed-hex <SIGNED_HEX>` — Signed and EIP-2718-encoded transaction hex (without `0x` prefix) - Begins with `02` for EIP-1559 transactions - Begins with `f8` for Legacy transactions
* `--rpc-url <RPC_URL>` — JSON-RPC endpoint URL



## `eth-offline-signer confirm`

Wait until a transaction is first included in a block and print its receipt

**Usage:** `eth-offline-signer confirm --tx-hash <TX_HASH> --rpc-url <RPC_URL>`

###### **Options:**

* `--tx-hash <TX_HASH>` — 0x-prefixed transaction hash to monitor
* `--rpc-url <RPC_URL>` — JSON-RPC endpoint URL for polling (or use RPC_URL env var)



## `eth-offline-signer markdown-help`

Output CLI documentation in Markdown format

**Usage:** `eth-offline-signer markdown-help`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

