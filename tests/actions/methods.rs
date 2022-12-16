use fuels::prelude::*;
use std::str::FromStr;

use crate::actions::setup::setup;

const USDT_ADDRESS: &str = "0x2cffcbc96717e5a102db1d5da45c189248d00a070cd65a822096b9733d3b071e";

#[tokio::test]
async fn initialize() {
    let (wallet, instance, _) = init().await;
    let decimals = instance
        .methods()
        .decimals()
        .simulate()
        .await
        .unwrap()
        .value;
    let symbol = instance.methods().symbol().simulate().await.unwrap().value;
    println!("Decimals: {decimals}\nSymbol: {symbol}");
    let result = instance
        .methods()
        .initialize(parse_units(1000, decimals), Address::from(wallet.address()))
        .tx_params(TxParameters::new(Some(1), Some(1000000), None))
        .call()
        .await;
    println!("{} Initialize", if result.is_ok() { "✅" } else { "❌" });
    let mint_amount = instance
        .methods()
        .get_mint_amount()
        .simulate()
        .await
        .unwrap();
    println!(
        "Mint amount {} {symbol}",
        format_units(mint_amount.value, decimals)
    );
}

#[tokio::test]
async fn mint() {
    let (wallet, dapp, _) = init().await;
    let decimals = dapp.methods().decimals().simulate().await.unwrap().value;
    let symbol = dapp.methods().symbol().simulate().await.unwrap().value;
    let mint_amount = dapp.methods().get_mint_amount().simulate().await.unwrap();
    let asset_id = AssetId::from_str(USDT_ADDRESS).unwrap();

    println!(
        "Decimals: {decimals}\nSymbol: {symbol}\nMint amount: {} {symbol}",
        format_units(mint_amount.value, decimals)
    );

    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}",
        format_units(balance, decimals)
    );
    let params = TxParameters::new(Some(1), Some(1000000), None);
    let result = dapp
        .methods()
        .mint()
        .append_variable_outputs(1)
        .tx_params(params)
        .call()
        .await;
    println!("{} Mint", if result.is_ok() { "✅" } else { "❌" });

    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}",
        format_units(balance, decimals)
    )
}

#[tokio::test]
async fn transfer() {
    let (wallet, dapp, provider) = setup().await;
    let decimals = dapp.methods().decimals().simulate().await.unwrap().value;
    let symbol = dapp.methods().symbol().simulate().await.unwrap().value;
    let asset_id = AssetId::from_str(USDT_ADDRESS).unwrap();

    println!("Decimals: {decimals}\nSymbol: {symbol}");

    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}",
        format_units(balance, decimals)
    );

    let recipient =
        Bech32Address::from_str("fuel1v3hdp7mpsy3mnsdy4jhwt4yk67n3yqgrn6mt0d4v3wvny2dn7f7sgf3ymm")
            .unwrap();
    let recipient = Wallet::from_address(recipient, Some(provider.clone()));

    let amount = parse_units(10, decimals);
    let mut inputs = vec![];
    let mut outputs = vec![];

    let input = wallet
        .get_asset_inputs_for_amount(asset_id, amount, 0)
        .await
        .unwrap();
    inputs.extend(input);

    let output = wallet.get_asset_outputs_for_amount(recipient.address(), asset_id, amount);
    outputs.extend(output);

    let mut tx = Wallet::build_transfer_tx(
        &inputs,
        &outputs,
        TxParameters::new(Some(1), Some(1000000), None),
    );
    wallet.sign_transaction(&mut tx).await.unwrap();

    let _receipts = provider.send_transaction(&tx).await.unwrap();

    let recipient_balance = recipient.get_asset_balance(&asset_id).await.unwrap();
    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}\nRecipient balance: {} {symbol}",
        format_units(balance, decimals),
        format_units(recipient_balance, decimals),
    )
}
