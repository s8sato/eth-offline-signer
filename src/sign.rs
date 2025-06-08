use std::marker::PhantomData;

use super::*;
use alloy::{
    consensus::{self, SignableTransaction, Signed},
    eips::Encodable2718,
    network::TxSignerSync,
    primitives::{Address, TxKind},
    signers::{Signature, local::PrivateKeySigner},
};

/// Errors for offline transaction signing.
#[derive(Display, Error, Debug)]
pub enum Error {
    /// Signing failed: {0}
    Sign(#[from] eyre::Report),
}

/// Common fields shared by all transaction payloads.
#[derive(Debug, Clone)]
pub struct CommonPayload {
    /// EIP-155 chain ID for replay protection (e.g., 1 for Mainnet, 11155111 for Sepolia).
    pub chain_id: u64,
    /// Transaction nonce: the sender's account transaction count at time of signing.
    pub nonce: u64,
    /// Maximum amount of gas units the transaction is allowed to consume.
    pub gas_limit: u64,
    /// Recipient address of the transaction.
    pub to: Address,
    /// Amount of Wei to transfer in this transaction.
    pub value: Wei,
}

/// Additional parameters for EIP-1559 (Type-2) transactions.
pub struct Eip1559Payload {
    /// Maximum total fee per gas in Wei.
    pub max_fee_per_gas: u128,
    /// Maximum priority fee (tip) per gas in Wei.
    pub max_priority_fee_per_gas: u128,
}

/// Additional parameter for legacy (pre-EIP-1559) transactions.
pub struct LegacyPayload {
    /// Gas price per unit in Wei.
    pub gas_price: u128,
}

/// A typed wrapper around a transaction in its unsigned state.
pub struct Tx<T>(T);

/// Marker trait to associate a transaction type with its unique payload.
pub trait Unique {
    /// The payload type carrying the unique parameters for this transaction.
    type UniquePayload;
}

/// Builds a transaction of type `T` by combining a common payload with type-specific data.
pub trait Build<T: Unique> {
    /// Combine this builder and the given unique payload to form a `Tx<T>`.
    fn build(self, unique: T::UniquePayload) -> Tx<T>;
}

impl<T: SignableTransaction<Signature>> Tx<T> {
    /// Sign the transaction using the provided private key signer.
    ///
    /// Consumes the unsigned `Tx<T>`, applies the digital signature,
    /// and returns a `TxSigned<T>` which can be encoded or submitted.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Sign`] if the signing operation fails.
    pub fn sign(mut self, signer: &PrivateKeySigner) -> Result<TxSigned<T>, Error> {
        let signature =
            signer.sign_transaction_sync(&mut self.0).map_err(|e| Error::Sign(eyre::eyre!(e)))?;
        let signed = self.0.into_signed(signature);
        Ok(TxSigned(signed))
    }
}

impl<T> TxSigned<T>
where
    Signed<T>: Encodable2718,
{
    /// Encode the signed transaction into the EIP-2718 envelope format.
    pub fn encode_2718(self) -> TxEip2718Bytes<T> {
        let mut buf = Vec::new();
        self.0.encode_2718(&mut buf);
        let bytes = core::mem::take(&mut buf);
        TxEip2718Bytes(bytes, PhantomData)
    }
}

impl Unique for consensus::TxEip1559 {
    type UniquePayload = Eip1559Payload;
}

impl Unique for consensus::TxLegacy {
    type UniquePayload = LegacyPayload;
}

impl Build<consensus::TxEip1559> for CommonPayload {
    fn build(self, unique: Eip1559Payload) -> Tx<consensus::TxEip1559> {
        let Self { chain_id, nonce, gas_limit, to, value } = self;
        let Eip1559Payload { max_fee_per_gas, max_priority_fee_per_gas } = unique;

        Tx(consensus::TxEip1559 {
            chain_id,
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            to: TxKind::Call(to),
            value,
            ..Default::default()
        })
    }
}

impl Build<consensus::TxLegacy> for CommonPayload {
    fn build(self, unique: LegacyPayload) -> Tx<consensus::TxLegacy> {
        let Self { chain_id, nonce, gas_limit, to, value } = self;
        let LegacyPayload { gas_price } = unique;

        Tx(consensus::TxLegacy {
            chain_id: Some(chain_id),
            nonce,
            gas_price,
            gas_limit,
            to: TxKind::Call(to),
            value,
            ..Default::default()
        })
    }
}
