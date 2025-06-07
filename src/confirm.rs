use super::*;
use alloy::network::Ethereum;
use alloy::primitives::TxHash;
use alloy::providers::{PendingTransactionBuilder, RootProvider};
use alloy::rpc::types::eth;

/// Errors for the first transaction confirmation.
#[derive(Display, Error, Debug)]
pub enum ConfirmError {
    /// Failed to retrieve the transaction receipt from the RPC endpoint: {0}
    Receipt(eyre::Report),
}

/// Retrieve the transaction receipt for a given hash from a JSON-RPC endpoint.
///
/// This function will query the provided RPC URL for the transaction receipt
/// and return it once available. It does not perform any retry logic beyond
/// the underlying builder behavior.
///
/// # Arguments
///
/// * `tx_hash` – The hash of the transaction to monitor.
/// * `rpc_url` – The JSON-RPC endpoint URL to query.
///
/// # Returns
///
/// On success, returns the `TransactionReceipt` containing block inclusion details.
///
/// # Errors
///
/// Returns `ConfirmError::Receipt` if the RPC call to fetch the receipt fails.
pub async fn get_receipt(
    tx_hash: TxHash,
    rpc_url: url::Url,
) -> Result<eth::TransactionReceipt, ConfirmError> {
    let provider = RootProvider::<Ethereum>::new_http(rpc_url);
    let pending = PendingTransactionBuilder::new(provider, tx_hash);
    let receipt = pending.get_receipt().await.map_err(|e| ConfirmError::Receipt(eyre::eyre!(e)))?;
    Ok(receipt)
}
