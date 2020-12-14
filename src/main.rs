use ethers::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //use std::convert::TryFrom;
    //let provider = Provider::<Http>::try_from("http://localhost:8545")?;

    let provider = Provider::new(Ws::connect("ws://localhost:8545").await?);

    let wallet = "0000000000000000000000000000000000000000000000000000000000Facade"
        .parse::<LocalWallet>()?;
    let bank = SignerMiddleware::new(provider.clone(), wallet);

    let accounts = bank.get_accounts().await?;
    dbg!(&accounts);

    let b0 = provider.get_balance(accounts[0], None).await?;
    dbg!(b0);
    let b1 = provider.get_balance(accounts[1], None).await?;
    dbg!(b1);

    let tx = TransactionRequest::new()
        .from(accounts[1])
        .to(accounts[0])
        .value(1);
    dbg!(&tx);

    let tx_hash = bank.send_transaction(tx, None).await?;
    dbg!(tx_hash);

    println!("Pending the transaction...");
    let receipt = bank.pending_transaction(tx_hash).await?;
    dbg!(&receipt);

    let tx = bank.get_transaction(receipt.transaction_hash).await?;
    dbg!(tx);

    Ok(())
}
