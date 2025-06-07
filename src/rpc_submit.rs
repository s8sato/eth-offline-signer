use super::*;
use alloy::network::Ethereum;
use alloy::primitives::TxHash;
use alloy::providers::{Provider, RootProvider};

/// Errors for raw transaction submission.
#[derive(Display, Error, Debug)]
pub enum SubmitError {
    /// RPC call failed when sending the raw transaction: {0}
    Rpc(eyre::Report),
}

/// Submit a signed raw transaction via JSON-RPC and return its transaction hash.
///
/// # Arguments
///
/// * `rlp_bytes` – RLP-encoded transaction bytes (signed raw transaction).
/// * `rpc_url` – JSON-RPC endpoint URL.
///
/// # Returns
///
/// On success, returns the `TxHash` of the submitted transaction.
///
/// # Errors
///
/// Returns `SubmitError::Rpc` if the RPC call to send the transaction fails.
pub async fn submit_raw(rlp_bytes: &[u8], rpc_url: url::Url) -> Result<TxHash, SubmitError> {
    let provider = RootProvider::<Ethereum>::new_http(rpc_url);
    let pending = provider
        .send_raw_transaction(rlp_bytes)
        .await
        .map_err(|e| SubmitError::Rpc(eyre::eyre!(e)))?;
    Ok(*pending.tx_hash())
}
