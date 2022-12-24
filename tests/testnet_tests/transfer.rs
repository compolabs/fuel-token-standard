use fuels::prelude::*;
use std::str::FromStr;

use crate::utils::{number_utils::*, testnet_tests_utils::setup};

const TOKEN_ADDRESS: &str = "YOUR TOKEN ADDRESS";
const TRANSFER_AMOUNT: u64 = 10; //TRANSFER AMOUNT WITHOUT DECIMALS
const RECIPIEND_ADDRES: &str = "RECIPIENT ADDRESS";

#[tokio::test]
async fn transfer() {
    let (wallet, dapp, provider) = setup(TOKEN_ADDRESS).await;
    let config = dapp.methods().config().simulate().await.unwrap().value;
    let decimals = config.decimals;
    let symbol = config.symbol;
    let asset_id = AssetId::from_str(TOKEN_ADDRESS).unwrap();

    println!("Decimals: {decimals}\nSymbol: {symbol}");

    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}",
        format_units(balance, decimals)
    );

    let recipient = Bech32Address::from_str(RECIPIEND_ADDRES).unwrap();
    let recipient = Wallet::from_address(recipient, Some(provider.clone()));

    let amount = parse_units(TRANSFER_AMOUNT, decimals);
    let _receipts = wallet
        .transfer(
            recipient.address(),
            amount,
            asset_id,
            TxParameters::new(Some(1), Some(1000000), None),
        )
        .await
        .unwrap();

    let recipient_balance = recipient.get_asset_balance(&asset_id).await.unwrap();
    let balance = wallet.get_asset_balance(&asset_id).await.unwrap();
    println!(
        "Wallet balance: {} {symbol}\nRecipient balance: {} {symbol}",
        format_units(balance, decimals),
        format_units(recipient_balance, decimals),
    )
}
