use fuels::prelude::*;

use crate::utils::{number_utils::*, testnet_tests_utils::*};

const ADDRESS: &str = "YOUR TOKEN ADDRESS";
const NAME: &str = "Bitcoin"; // YOUR TOKEN NAME
const SYMBOL: &str = "BTC"; // YOUR TOKEN SYMBOL
const DECIMALS: u8 = 8; // YOUR TOKEN DECIMALS

#[tokio::test]
async fn initialize() {
    let (wallet, instance, _provider) = setup(ADDRESS).await;
    let mut name = NAME.to_string().clone();
    let mut symbol = SYMBOL.to_string().clone();
    name.push_str(" ".repeat(32 - name.len()).as_str());
    symbol.push_str(" ".repeat(8 - symbol.len()).as_str());
    let result = instance
        .methods()
        .initialize(
            TokenInitializeConfig {
                name: fuels::core::types::SizedAsciiString::<32>::new(String::from(name)).unwrap(),
                symbol: fuels::core::types::SizedAsciiString::<8>::new(String::from(symbol))
                    .unwrap(),
                decimals: DECIMALS,
            },
            parse_units(1000, DECIMALS),
            Identity::Address(Address::from(wallet.address())),
        )
        .tx_params(TxParameters::new(Some(1), Some(1000000), None))
        .call()
        .await;
    println!("{} Initialize", if result.is_ok() { "✅" } else { "❌" });

    let config = instance.methods().config().simulate().await.unwrap().value;
    let decimals = config.decimals;
    let symbol = config.symbol;
    println!("Decimals: {decimals}\nSymbol: {symbol}");

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
