use fuels::prelude::*;
use crate::{actions::setup::setup, utils::{parse_units, format_units}};

// const USDT_ADDRESS: &str = "0x777923117c7772c0680806d2a0d3a0eb5e654fa65e48d8de85516f6f85ba4887";

#[tokio::test]
async fn initialize() {
    let (wallet, instance, _) = setup().await;
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
