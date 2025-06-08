use alloy::{
    primitives::{Address, TxHash, utils},
    signers::local::PrivateKeySigner,
};

use clap::{Parser, Subcommand};
use color_eyre::eyre;

use eth_offline_signer as lib;

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

        /// Chain ID (1=mainnet, 5=goerli, etc.)
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
        tx_type_args: TxTypeArgs,
    },

    /// Submit a previously signed raw transaction via JSON-RPC
    Submit {
        /// 0x-prefixed signed raw transaction hex
        #[arg(long)]
        signed_tx_hex: String,

        /// JSON-RPC endpoint URL (can also be set via RPC_URL env var)
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
enum TxTypeArgs {
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

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::Sign { private_key, chain_id, nonce, gas_limit, to, value, tx_type_args } => {
            let rlp_bytes = lib::TxTypeArgs::from(tx_type_args).sign_tx_into_rlp_bytes(
                &private_key,
                chain_id,
                nonce,
                gas_limit,
                to,
                value,
            )?;
            let signed_tx_hex = hex::encode(rlp_bytes);
            println!("{signed_tx_hex}")
        }
        Command::Submit { signed_tx_hex, rpc_url } => {
            let rlp_bytes = hex::decode(signed_tx_hex)?;
            let tx_hash = lib::submit_raw(&rlp_bytes, rpc_url).await?;
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

impl From<TxTypeArgs> for lib::TxTypeArgs {
    fn from(value: TxTypeArgs) -> Self {
        match value {
            TxTypeArgs::Eip1559 { max_fee_per_gas, max_priority_fee_per_gas } => {
                lib::TxTypeArgs::Eip1559(lib::Eip1559Args {
                    max_fee_per_gas,
                    max_priority_fee_per_gas,
                })
            }
            TxTypeArgs::Legacy { gas_price } => {
                lib::TxTypeArgs::Legacy(lib::LegacyArgs { gas_price })
            }
        }
    }
}
