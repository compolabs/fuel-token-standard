use dotenv::dotenv;
use fuels::prelude::*;
use std::str::FromStr;

abigen!(UsdtContract, "out/debug/token_contract-abi.json");

const RPC: &str = "node-beta-2.fuel.network";
const USDT_ADDRESS: &str = "0x2cffcbc96717e5a102db1d5da45c189248d00a070cd65a822096b9733d3b071e";

async fn init() -> (WalletUnlocked, UsdtContract) {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let usdt_dapp_id = Bech32ContractId::from(ContractId::from_str(USDT_ADDRESS).unwrap());
    let usdt_dapp_instance = UsdtContract::new(usdt_dapp_id, wallet.clone());

    println!("👛 Account address     @ {}", wallet.clone().address());
    println!(
        "🗞  USDT dapp address   @ {}",
        usdt_dapp_instance.get_contract_id()
    );
    return (wallet, usdt_dapp_instance);
}

#[tokio::test]
async fn initialize() {
    let (wallet, instance) = init().await;
    let decimals = instance.methods().decimals().call().await.unwrap().value;
    let symbol = instance.methods().symbol().call().await.unwrap().value;

    instance
        .methods()
        .initialize(parse_units(1000, decimals), Address::from(wallet.address()))
        .tx_params(TxParameters::new(Some(1), Some(1000000), None))
        .call()
        .await
        .unwrap();
    let mint_amount = instance.methods().get_mint_amount().call().await.unwrap();
    println!(
        "Mint amount {} {symbol}",
        format_units(mint_amount.value, decimals)
    );
}

fn parse_units(num: u64, decimals: u8) -> u64 {
    num * 10u64.pow(decimals as u32)
}

fn format_units(num: u64, decimals: u8) -> u64 {
    num / 10u64.pow(decimals as u32)
}
