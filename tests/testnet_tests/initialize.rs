use fuels::prelude::*;

use crate::utils::{number_utils::*, testnet_tests_utils::*};

const ADDRESS: &str = "YOUR TOKEN ADDRESS";

#[tokio::test]
async fn initialize() {
    let (wallet, instance, _provider) = setup(ADDRESS).await;
    let config = instance.methods().config().simulate().await.unwrap().value;
    let decimals = config.decimals;
    let symbol = config.symbol;
    println!("Decimals: {decimals}\nSymbol: {symbol}");
    let result = instance
        .methods()
        .initialize(
            TokenInitializeConfig {
                name: fuels::core::types::SizedAsciiString::<32>::new(String::from(
                    "Big Black Coin                  ",
                ))
                .unwrap(),
                symbol: fuels::core::types::SizedAsciiString::<8>::new(String::from("BBC     "))
                    .unwrap(),
                decimals,
            },
            parse_units(1000, decimals),
            Address::from(wallet.address()),
        )
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
