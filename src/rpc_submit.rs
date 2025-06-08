use super::*;
use alloy::consensus::{Signed, TxEip1559, TxEnvelope};
use alloy::network::Ethereum;
use alloy::primitives::TxHash;
use alloy::providers::{Provider, RootProvider};

/// Errors for raw transaction submission.
#[derive(Display, Error, Debug)]
pub enum SubmitError {
    /// RPC call failed when sending the raw transaction: {0}
    Rpc(eyre::Report),
    /// Failed to decode the EIP-2718 encoded transaction bytes into a typed transaction: {0}
    Decode(eyre::Report),
}

/// Submit a signed raw transaction via JSON-RPC and return its transaction hash.
///
/// # Arguments
///
/// * `eip2718_bytes` – EIP-2718-encoded transaction bytes.
/// * `rpc_url` – JSON-RPC endpoint URL.
///
/// # Returns
///
/// On success, returns the `TxHash` of the submitted transaction.
///
/// # Errors
///
/// Returns `SubmitError::Rpc` if the RPC call to send the transaction fails.
pub async fn submit_raw(
    mut eip2718_bytes: &[u8],
    rpc_url: url::Url,
) -> Result<TxHash, SubmitError> {
    let tx_envelope: TxEnvelope = Signed::<TxEip1559>::eip2718_decode(&mut eip2718_bytes)
        .map(Into::into)
        .map_err(|e| SubmitError::Decode(eyre::eyre!(e)))?;
    let provider = RootProvider::<Ethereum>::new_http(rpc_url);
    let pending = provider
        .send_tx_envelope(tx_envelope)
        .await
        .map_err(|e| SubmitError::Rpc(eyre::eyre!(e)))?;
    Ok(*pending.tx_hash())
}
