use std::marker::PhantomData;

use alloy::consensus::Signed;
use color_eyre::eyre;
use displaydoc::Display;
use thiserror::Error;

pub mod confirm;
pub mod sign;
pub mod submit;

pub use alloy::primitives::U256 as Wei;
pub use confirm::get_receipt;
pub use sign::{CommonPayload, Eip1559Payload, LegacyPayload};

/// Wrapper type indicating a transaction has been signed.
pub struct TxSigned<T>(Signed<T>);

/// Container for an EIP-2718 envelope–encoded signed transaction.
pub struct TxEip2718Bytes<T>(Vec<u8>, std::marker::PhantomData<T>);

impl<T> AsRef<[u8]> for TxEip2718Bytes<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<T> TxEip2718Bytes<T> {
    /// Construct a typed `TxEip2718Bytes` wrapper from raw RLP bytes.
    pub fn from_untyped(bytes: Vec<u8>) -> Self {
        Self(bytes, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use crate::sign::{Build, CommonPayload};

    use super::*;

    use alloy::{
        consensus::{TxEip1559, TxEnvelope, TxLegacy},
        eips::Decodable2718,
        node_bindings::Anvil,
        primitives::U256,
        providers::{Provider, ProviderBuilder},
        signers::local::PrivateKeySigner,
    };

    #[tokio::test]
    async fn smoke_test_e1559() -> eyre::Result<()> {
        fn callback_sign(
            common_payload: CommonPayload,
            signer: &PrivateKeySigner,
        ) -> eyre::Result<TxEip2718Bytes<TxEip1559>> {
            let payload = Eip1559Payload {
                max_fee_per_gas: 20_000_000_000,
                max_priority_fee_per_gas: 1_000_000_000,
            };
            let signed_bytes = common_payload.clone().build(payload).sign(signer)?.encode_2718();
            Ok(signed_bytes)
        }
        smoke_test::<TxEip1559>(callback_sign).await?;

        Ok(())
    }

    #[tokio::test]
    async fn smoke_test_legacy() -> eyre::Result<()> {
        fn callback_sign(
            common_payload: CommonPayload,
            signer: &PrivateKeySigner,
        ) -> eyre::Result<TxEip2718Bytes<TxLegacy>> {
            let payload = LegacyPayload { gas_price: 20_000_000_000 };
            let signed_bytes = common_payload.clone().build(payload).sign(signer)?.encode_2718();
            Ok(signed_bytes)
        }
        smoke_test::<TxLegacy>(callback_sign).await?;

        Ok(())
    }

    /// Smoke test: offline sign, submit to Anvil, and confirm receipt
    async fn smoke_test<T>(
        callback_sign: impl FnOnce(CommonPayload, &PrivateKeySigner) -> eyre::Result<TxEip2718Bytes<T>>,
    ) -> eyre::Result<()>
    where
        Signed<T>: Decodable2718,
        TxEnvelope: From<Signed<T>>,
    {
        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let anvil = Anvil::new().block_time(1).try_spawn()?;

        // Set up signer from the first default Anvil account (Alice).
        // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
        // The following code is for testing only. Set up signer from private key, be aware of danger.
        // let signer: PrivateKeySigner = "<PRIVATE_KEY>".parse().expect("should parse private key");
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();

        // Once call the RPC endpoint to get the nonce.
        let rpc_url = anvil.endpoint_url();
        println!("Anvil endpoint URL: {rpc_url}");
        let provider = ProviderBuilder::new().wallet(signer.clone()).connect_http(rpc_url.clone());
        let accounts = provider.get_accounts().await?;
        let alice = accounts[0];
        let bob = accounts[1];
        let nonce = provider.get_transaction_count(alice).await.unwrap();

        // DISCONNECT and sign offline.
        println!("Signing transaction");
        let common_payload = CommonPayload {
            chain_id: anvil.chain_id(),
            nonce,
            gas_limit: 21_000,
            to: bob,
            value: U256::from(100),
        };
        let signed_bytes = callback_sign(common_payload, &signer)?;
        println!("Signed transaction: {}", hex::encode(&signed_bytes));

        // RECONNECT and submit transaction.
        let tx_hash = signed_bytes.decode_2718()?.submit(rpc_url.clone()).await?;
        println!("Submitted transaction: {tx_hash}");

        // Confirm the transaction receipt.
        let tx_receipt = get_receipt(tx_hash, rpc_url).await?;
        println!("Got transaction receipt: {tx_receipt:#?}");

        assert_eq!(tx_receipt.from, alice);
        assert_eq!(tx_receipt.to, Some(bob));

        Ok(())
    }
}
