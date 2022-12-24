use fuels::prelude::*;
use std::str::FromStr;

use crate::utils::{testnet_tests_utils::setup, number_utils::parse_units};

abigen!(Market, "tests/artefacts/market/market-abi.json");

const TOKEN_ADDRESS: &str = "YOUR TOKEN ADDRESS";
const MARKET_ADDRESS: &str = "0xabc9873302b555db02ad2e79a4afb6b1d2acbab1bf4fe08149aff8fc904a8e0c";

#[tokio::test]
async fn merket_interact() {
    let (wallet, dapp, _provider) = setup(TOKEN_ADDRESS).await;
    let config = dapp.methods().config().simulate().await.unwrap().value;
    let decimals = config.decimals;
    let symbol = config.symbol;

    let asset_id = AssetId::from_str(TOKEN_ADDRESS).unwrap();

    let market_dapp_id = Bech32ContractId::from(ContractId::from_str(MARKET_ADDRESS).unwrap());
    let market_dapp = Market::new(market_dapp_id, wallet.clone());
    let lp_asset_id = AssetId::from(*market_dapp.get_contract_id().hash());

    println!("\nDecimals   : {decimals}\nSymbol     : {symbol}\nAsset id   : {asset_id}\nLP asset id: {asset_id}\n");
    print_balances(&wallet).await;

    let deposit_amount = parse_units(10, decimals);

    let tx_params = TxParameters::new(Some(1), Some(1000000), None);
    let call_params = CallParameters::new(Some(deposit_amount), Some(asset_id), None);
    let _result = market_dapp
        .methods()
        .supply()
        .call_params(call_params)
        .tx_params(tx_params)
        .append_variable_outputs(1)
        .call()
        .await;
    println!("{} Supply\n", if _result.is_ok() { "✅" } else { "❌" });

    print_balances(&wallet).await;

    let lp_token_balance = wallet.get_asset_balance(&lp_asset_id).await.unwrap();
    let call_params = CallParameters::new(Some(lp_token_balance), Some(lp_asset_id), None);
    let _result = market_dapp
        .methods()
        .withdraw()
        .tx_params(tx_params)
        .call_params(call_params)
        .append_variable_outputs(1)
        .call()
        .await;
    println!("{} Withdraw\n", if _result.is_ok() { "✅" } else { "❌" });

    print_balances(&wallet).await;
}

async fn print_balances(wallet: &WalletUnlocked) {
    let balances = wallet.get_balances().await.unwrap();
    println!("{:#?}\n", balances);
}
