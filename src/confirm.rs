use super::*;
use alloy::network::Ethereum;
use alloy::primitives::TxHash;
use alloy::providers::{PendingTransactionBuilder, RootProvider};
use alloy::rpc::types::eth;

/// Errors for the first transaction confirmation.
#[derive(Display, Error, Debug)]
pub enum Error {
    /// Failed to retrieve the transaction receipt from the RPC endpoint: {0}
    Receipt(eyre::Report),
}

/// Retrieve the transaction receipt for a given hash from a JSON-RPC endpoint.
///
/// # Errors
///
/// Returns [`Error::Receipt`] if the RPC call to fetch the receipt fails.
pub async fn get_receipt(
    tx_hash: TxHash,
    rpc_url: url::Url,
) -> Result<eth::TransactionReceipt, Error> {
    let provider = RootProvider::<Ethereum>::new_http(rpc_url);
    let pending = PendingTransactionBuilder::new(provider, tx_hash);
    let receipt = pending.get_receipt().await.map_err(|e| Error::Receipt(eyre::eyre!(e)))?;
    Ok(receipt)
}
