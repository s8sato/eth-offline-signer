use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
#[ignore = "Requires Anvil running on localhost:8545"]
fn cli_test() -> Result<(), Box<dyn Error>> {
    println!("Running cli_test_eip1559 ...");
    cli_test_eip1559()?;
    println!("Running cli_test_legacy ...");
    cli_test_legacy()?;
    Ok(())
}

fn cli_test_eip1559() -> Result<(), Box<dyn Error>> {
    // 1) Offline sign
    let mut cmd_sign = Command::cargo_bin("eth-offline-signer")?;
    let assert1 = cmd_sign
        .args([
            "sign",
            "--private-key",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "--chain-id",
            "31337",
            "--nonce",
            "0",
            "--gas-limit",
            "21000",
            "--to",
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
            "--eth",
            "0.001",
            "eip1559",
            "--max-fee-per-gas",
            "20000000000",
            "--max-priority-fee-per-gas",
            "1000000000",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("02f874827a6980843b9aca008504a817c8008252089470997970c51812dc3a010c7d01b50e0d17dc79c887038d7ea4c6800080c001a09044137087a42645941a32f3b0911283efeb8b986a6cba22e0fb56bd366a28b2a00e175b4068e37dbf60b1398151dfba93330937a837d49aba569aed93515f87de"));
    let stdout1 = String::from_utf8(assert1.get_output().stdout.clone())?;
    let signed_hex = stdout1.trim();
    println!("Signed transaction. EIP-2718 envelope:");
    println!("{signed_hex}");

    // 2) Submit to Anvil
    let mut cmd_submit = Command::cargo_bin("eth-offline-signer")?;
    let assert2 = cmd_submit
        .args([
            "submit",
            "eip1559",
            "--signed-hex",
            signed_hex,
            "--rpc-url",
            "http://localhost:8545",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "0xab1837a711aad971e2c1f65b6cb768703d6b03b6b7a2bfd685fda5a4cd757c3e",
        ));
    let stdout2 = String::from_utf8(assert2.get_output().stdout.clone())?;
    let tx_hash = stdout2.trim();
    println!("Submitted transaction. Hash:");
    println!("{tx_hash}");

    // 3) Wait for confirmation
    let mut cmd_confirm = Command::cargo_bin("eth-offline-signer")?;
    let assert3 = cmd_confirm
        .args(["confirm", "--tx-hash", tx_hash, "--rpc-url", "http://localhost:8545"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "status: Eip658(
                    true,
                )",
        ));
    let stdout3 = String::from_utf8(assert3.get_output().stdout.clone())?;
    let tx_receipt = stdout3.trim();
    println!("Confirmed transaction. Receipt:");
    println!("{tx_receipt}");

    Ok(())
}

fn cli_test_legacy() -> Result<(), Box<dyn Error>> {
    // 1) Offline sign
    let mut cmd_sign = Command::cargo_bin("eth-offline-signer")?;
    let assert1 = cmd_sign
        .args([
            "sign",
            "--private-key",
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "--chain-id",
            "31337",
            "--nonce",
            "1",
            "--gas-limit",
            "21000",
            "--to",
            "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
            "--eth",
            "0.001",
            "legacy",
            "--gas-price",
            "20000000000",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("f86d018504a817c8008252089470997970c51812dc3a010c7d01b50e0d17dc79c887038d7ea4c680008082f4f5a0f9a5bcd36a8edb13fea3b69bb654b9c355c98bc34c396956e8e922ac56d97b24a03e665dbb8fcfcf28dda49e354c338ead14f38ac664b76cdd01ceaa7488eea6a2"));
    let stdout1 = String::from_utf8(assert1.get_output().stdout.clone())?;
    let signed_hex = stdout1.trim();
    println!("Signed transaction. EIP-2718 envelope:");
    println!("{signed_hex}");

    // 2) Submit to Anvil
    let mut cmd_submit = Command::cargo_bin("eth-offline-signer")?;
    let assert2 = cmd_submit
        .args([
            "submit",
            "legacy",
            "--signed-hex",
            signed_hex,
            "--rpc-url",
            "http://localhost:8545",
        ])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "0xd1ea22c07a711fcd438a60da1eaa7dbe683a9b852780d5caef2badd9d9a85c05",
        ));
    let stdout2 = String::from_utf8(assert2.get_output().stdout.clone())?;
    let tx_hash = stdout2.trim();
    println!("Submitted transaction. Hash:");
    println!("{tx_hash}");

    // 3) Wait for confirmation
    let mut cmd_confirm = Command::cargo_bin("eth-offline-signer")?;
    let assert3 = cmd_confirm
        .args(["confirm", "--tx-hash", tx_hash, "--rpc-url", "http://localhost:8545"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "status: Eip658(
                    true,
                )",
        ));
    let stdout3 = String::from_utf8(assert3.get_output().stdout.clone())?;
    let tx_receipt = stdout3.trim();
    println!("Confirmed transaction. Receipt:");
    println!("{tx_receipt}");

    Ok(())
}
