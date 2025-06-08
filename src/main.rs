use alloy::{
    consensus::{TxEip1559, TxLegacy},
    primitives::{Address, TxHash, utils},
    signers::local::PrivateKeySigner,
};

use clap::{Parser, Subcommand, ValueEnum};
use color_eyre::eyre;

use eth_offline_signer::{self as lib, Eip1559Payload, LegacyPayload, TxEip2718Bytes, sign::Build};

/// CLI for offline signing and RPC submission of Ethereum-compatible transactions
#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Offline-only transaction signing (no network calls)
    Sign {
        /// 0x-prefixed private key for signing
        #[arg(long, env = "PRIVATE_KEY")]
        private_key: PrivateKeySigner,

        /// Chain ID (e.g. 1 for Mainnet, 5 for Goerli)
        #[arg(long)]
        chain_id: u64,

        /// Transaction nonce (pre-fetched from RPC)
        #[arg(long)]
        nonce: u64,

        /// Maximum gas units to allow for this transaction
        #[clap(long, default_value = "21000")]
        gas_limit: u64,

        /// 0x-prefixed recipient address
        #[arg(long)]
        to: Address,

        /// Amount to send in ETH (e.g. "0.01")
        #[arg(id = "eth", long, value_parser = utils::parse_ether)]
        value: lib::Wei,

        /// Specify fee model and parameters
        #[command(subcommand)]
        unique_args: UniqueArgs,
    },

    /// Submit a previously signed raw transaction via JSON-RPC
    Submit {
        /// Transaction type: EIP-1559 (Type 2) or Legacy (Type 0)
        #[arg(value_enum)]
        tx_type: TxType,

        /// Signed and EIP-2718-encoded transaction hex (without `0x` prefix)
        /// - Begins with `02` for EIP-1559 transactions
        /// - Begins with `f8` for Legacy transactions
        #[arg(long)]
        signed_hex: String,

        /// JSON-RPC endpoint URL
        #[arg(long, env = "RPC_URL")]
        rpc_url: url::Url,
    },

    /// Wait until a transaction is first included in a block and print its receipt
    Confirm {
        /// 0x-prefixed transaction hash to monitor
        #[arg(long)]
        tx_hash: TxHash,

        /// JSON-RPC endpoint URL for polling (or use RPC_URL env var)
        #[arg(long, env = "RPC_URL")]
        rpc_url: url::Url,
    },

    /// Output CLI documentation in Markdown format
    MarkdownHelp,
}

#[derive(Subcommand)]
enum UniqueArgs {
    /// Use the EIP-1559 fee market model
    Eip1559 {
        /// Maximum total fee per gas in Wei
        #[arg(long)]
        max_fee_per_gas: u128,

        /// Maximum priority fee per gas (tip) in Wei
        #[arg(long)]
        max_priority_fee_per_gas: u128,
    },

    /// Use the legacy gas price model
    Legacy {
        /// Gas price in Wei
        #[arg(long)]
        gas_price: u128,
    },
}

#[derive(ValueEnum, Clone)]
enum TxType {
    /// Use the EIP-1559 fee market (Type-2 transaction)
    Eip1559,
    /// Use the legacy gas price model (Type-0 transaction)
    Legacy,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::Sign { private_key, chain_id, nonce, gas_limit, to, value, unique_args } => {
            let common_payload = lib::CommonPayload { chain_id, nonce, gas_limit, to, value };
            let signed_hex = match unique_args {
                UniqueArgs::Eip1559 { max_fee_per_gas, max_priority_fee_per_gas } => {
                    let unique_payload =
                        Eip1559Payload { max_fee_per_gas, max_priority_fee_per_gas };
                    let signed_bytes: TxEip2718Bytes<TxEip1559> =
                        common_payload.build(unique_payload).sign(&private_key)?.encode_2718();
                    hex::encode(signed_bytes)
                }
                UniqueArgs::Legacy { gas_price } => {
                    let unique_payload = LegacyPayload { gas_price };
                    let signed_bytes: TxEip2718Bytes<TxLegacy> =
                        common_payload.build(unique_payload).sign(&private_key)?.encode_2718();
                    hex::encode(signed_bytes)
                }
            };
            println!("{signed_hex}")
        }
        Command::Submit { signed_hex, rpc_url, tx_type } => {
            let tx_hash = match tx_type {
                TxType::Eip1559 => {
                    let signed_bytes: TxEip2718Bytes<TxEip1559> =
                        hex::decode(signed_hex).map(TxEip2718Bytes::from_untyped)?;
                    let signed = signed_bytes.decode_2718()?;
                    signed.submit(rpc_url).await?
                }
                TxType::Legacy => {
                    let signed_bytes: TxEip2718Bytes<TxLegacy> =
                        hex::decode(signed_hex).map(TxEip2718Bytes::from_untyped)?;
                    let signed = signed_bytes.decode_2718()?;
                    signed.submit(rpc_url).await?
                }
            };
            println!("{tx_hash}");
        }
        Command::Confirm { tx_hash, rpc_url } => {
            let receipt = lib::get_receipt(tx_hash, rpc_url).await?;
            println!("{receipt:#?}");
        }
        Command::MarkdownHelp => clap_markdown::print_help_markdown::<Cli>(),
    }

    Ok(())
}
