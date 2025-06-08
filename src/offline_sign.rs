use super::*;
use alloy::{
    consensus::{self, SignableTransaction, Signed},
    network::TxSignerSync,
    primitives::{Address, TxKind},
    signers::{Signature, local::PrivateKeySigner},
};

/// Errors for offline transaction signing.
#[derive(Display, Error, Debug)]
pub enum SignError {
    /// Signing failed: {0}
    Sign(#[from] eyre::Report),
}

/// Transaction fee model and parameters.
pub enum TxTypeArgs {
    /// EIP-1559 fee market parameters.
    Eip1559(Eip1559Args),
    /// Legacy (pre-EIP-1559) gas price parameter.
    Legacy(LegacyArgs),
}

/// Parameters for EIP-1559 transaction signing.
pub struct Eip1559Args {
    /// Maximum total fee per gas in Gwei.
    pub max_fee_per_gas: u128,
    /// Maximum priority fee (tip) per gas in Gwei.
    pub max_priority_fee_per_gas: u128,
}

/// Parameters for legacy transaction signing.
pub struct LegacyArgs {
    /// Gas price in Gwei.
    pub gas_price: u128,
}

impl TxTypeArgs {
    /// Sign the transaction using the selected fee model and return EIP-2718-encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `self` – The fee model and its parameters.
    /// * `signer` – The private key signer.
    /// * `chain_id` – Chain ID for EIP-155 replay protection.
    /// * `nonce` – Transaction nonce.
    /// * `gas_limit` – Gas limit for the transaction.
    /// * `to` – Recipient address.
    /// * `value` – Amount to send (in Wei).
    ///
    /// # Returns
    ///
    /// A vector of EIP-2718-encoded transaction bytes, or a `SignError` if signing fails.
    pub fn sign_tx_into_eip2718_bytes(
        self,
        signer: &PrivateKeySigner,
        chain_id: u64,
        nonce: u64,
        gas_limit: u64,
        to: Address,
        value: Wei,
    ) -> Result<Vec<u8>, SignError> {
        let mut eip2718_buf = Vec::new();
        match self {
            TxTypeArgs::Eip1559(args) => args
                .sign_offline(signer, chain_id, nonce, gas_limit, to, value)?
                .eip2718_encode(&mut eip2718_buf),
            TxTypeArgs::Legacy(args) => args
                .sign_offline(signer, chain_id, nonce, gas_limit, to, value)?
                .eip2718_encode(&mut eip2718_buf),
        }
        let eip2718_bytes = core::mem::take(&mut eip2718_buf);
        Ok(eip2718_bytes)
    }
}

trait SignOffline: Sized {
    type Transaction: SignableTransaction<Signature>;

    fn sign_offline(
        self,
        signer: &PrivateKeySigner,
        chain_id: u64,
        nonce: u64,
        gas_limit: u64,
        to: Address,
        value: Wei,
    ) -> Result<Signed<Self::Transaction>, SignError> {
        let mut tx = self.build_transaction(chain_id, nonce, gas_limit, to, value);
        let signature =
            signer.sign_transaction_sync(&mut tx).map_err(|e| SignError::Sign(eyre::eyre!(e)))?;
        Ok(tx.into_signed(signature))
    }

    fn build_transaction(
        self,
        chain_id: u64,
        nonce: u64,
        gas_limit: u64,
        to: Address,
        value: Wei,
    ) -> Self::Transaction;
}

impl SignOffline for Eip1559Args {
    type Transaction = consensus::TxEip1559;
    fn build_transaction(
        self,
        chain_id: u64,
        nonce: u64,
        gas_limit: u64,
        to: Address,
        value: Wei,
    ) -> Self::Transaction {
        let Self { max_fee_per_gas, max_priority_fee_per_gas } = self;
        consensus::TxEip1559 {
            chain_id,
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            to: TxKind::Call(to),
            value,
            ..Default::default()
        }
    }
}

impl SignOffline for LegacyArgs {
    type Transaction = consensus::TxLegacy;
    fn build_transaction(
        self,
        chain_id: u64,
        nonce: u64,
        gas_limit: u64,
        to: Address,
        value: Wei,
    ) -> Self::Transaction {
        consensus::TxLegacy {
            chain_id: Some(chain_id),
            nonce,
            gas_price: self.gas_price,
            gas_limit,
            to: TxKind::Call(to),
            value,
            ..Default::default()
        }
    }
}
