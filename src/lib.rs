use color_eyre::eyre;
use displaydoc::Display;
use thiserror::Error;

mod confirm;
mod offline_sign;
mod rpc_submit;

pub use alloy::primitives::U256 as Wei;
pub use confirm::get_receipt;
pub use offline_sign::{Eip1559Args, LegacyArgs, TxTypeArgs};
pub use rpc_submit::submit_raw;

#[cfg(test)]
mod tests {
    use super::*;

    use alloy::{
        node_bindings::Anvil,
        primitives::U256,
        providers::{Provider, ProviderBuilder},
        signers::local::PrivateKeySigner,
    };

    /// Smoke test: offline sign, submit to Anvil, and confirm receipt
    #[tokio::test]
    async fn smoke_test_sign_submit_confirm() -> eyre::Result<()> {
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
        let eip1559_args = TxTypeArgs::Eip1559(Eip1559Args {
            max_fee_per_gas: 20_000_000_000,
            max_priority_fee_per_gas: 1_000_000_000,
        });
        let rlp_bytes = eip1559_args.sign_tx_into_rlp_bytes(
            &signer,
            anvil.chain_id(),
            nonce,
            21_000,
            bob,
            U256::from(100),
        )?;
        println!("Signed transaction: {}", hex::encode(rlp_bytes.as_slice()));

        // FIXME: EIP-1559 transaction submission fails with [Provider::send_raw_transaction()](https://docs.rs/alloy/1.0.9/alloy/providers/trait.Provider.html#method.send_raw_transaction).
        // let tx_hash = submit_raw(&rlp_bytes, rpc_url.clone()).await?;

        // Instead, try legacy transaction type.
        println!("Signing transaction");
        let legacy_args = TxTypeArgs::Legacy(LegacyArgs { gas_price: 20_000_000_000 });
        let rlp_bytes = legacy_args.sign_tx_into_rlp_bytes(
            &signer,
            anvil.chain_id(),
            nonce,
            21_000,
            bob,
            U256::from(100),
        )?;
        println!("Signed transaction: {}", hex::encode(rlp_bytes.as_slice()));

        // RECONNECT and submit transaction.
        let tx_hash = submit_raw(&rlp_bytes, rpc_url.clone()).await?;
        println!("Submitted transaction: {tx_hash}");

        // Confirm the transaction receipt.
        let tx_receipt = get_receipt(tx_hash, rpc_url).await?;
        println!("Got transaction receipt: {tx_receipt:#?}");

        assert_eq!(tx_receipt.from, alice);
        assert_eq!(tx_receipt.to, Some(bob));

        Ok(())
    }
}
