use super::*;
use alloy::consensus::{Signed, TxEnvelope};
use alloy::eips::Decodable2718;
use alloy::network::Ethereum;
use alloy::primitives::TxHash;
use alloy::providers::{Provider, RootProvider};

/// Errors for transaction submission.
#[derive(Display, Error, Debug)]
pub enum Error {
    /// Failed to decode the EIP-2718 encoded transaction bytes into a typed transaction: {0}
    Decode(eyre::Report),
    /// RPC call failed when sending the transaction: {0}
    Submit(eyre::Report),
}

impl<T> TxEip2718Bytes<T>
where
    Signed<T>: Decodable2718,
{
    /// Decode this EIP-2718 envelope–encoded byte sequence back into a signed transaction.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Decode`] if the byte sequence cannot be parsed as a valid
    /// EIP-2718 transaction for type `T`.
    pub fn decode_2718(self) -> Result<TxSigned<T>, Error> {
        let signed = Signed::<T>::decode_2718(&mut self.0.as_slice())
            .map_err(|e| Error::Decode(eyre::eyre!(e)))?;
        Ok(TxSigned(signed))
    }
}

impl<T> TxSigned<T>
where
    TxEnvelope: From<Signed<T>>,
{
    /// Submit this signed transaction to an Ethereum JSON-RPC endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Submit`] if the RPC node rejects or fails to process the transaction.
    pub async fn submit(self, rpc_url: url::Url) -> Result<TxHash, Error> {
        let tx_envelope: TxEnvelope = self.0.into();
        let provider = RootProvider::<Ethereum>::new_http(rpc_url);
        let pending = provider
            .send_tx_envelope(tx_envelope)
            .await
            .map_err(|e| Error::Submit(eyre::eyre!(e)))?;
        Ok(*pending.tx_hash())
    }
}
