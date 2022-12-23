use fuels::prelude::*;
use std::str::FromStr;

use crate::utils::{testnet_tests_utils::setup, number_utils::*};

const USDT_ADDRESS: &str = "YOUR TOKEN ADDRESS";

#[tokio::test]
async fn mint() {
    let (wallet, dapp, _provider) = setup(USDT_ADDRESS).await;
    let config = dapp.methods().config().simulate().await.unwrap().value;
    let decimals = config.decimals;
    let symbol = config.symbol;
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
