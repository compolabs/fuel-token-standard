use crate::{actions::setup::setup, utils::format_units};
use fuels::prelude::*;
use std::str::FromStr;

const USDT_ADDRESS: &str = "0x777923117c7772c0680806d2a0d3a0eb5e654fa65e48d8de85516f6f85ba4887";

#[tokio::test]
async fn mint() {
    let (wallet, dapp, _) = setup().await;
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