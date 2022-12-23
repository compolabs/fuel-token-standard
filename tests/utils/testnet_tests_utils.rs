use dotenv::dotenv;
use fuels::prelude::*;
use std::str::FromStr;

abigen!(TokenContract, "out/debug/token_contract-abi.json");

const RPC: &str = "node-beta-2.fuel.network";

pub async fn setup(address: &str) -> (WalletUnlocked, TokenContract, Provider) {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet =
        WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider.clone()));

    let usdt_dapp_id = Bech32ContractId::from(ContractId::from_str(address).unwrap());
    let usdt_dapp_instance = TokenContract::new(usdt_dapp_id, wallet.clone());

    println!("👛 Account address     @ {}", wallet.clone().address());
    println!(
        "🗞  USDT dapp address   @ {}",
        usdt_dapp_instance.get_contract_id()
    );
    return (wallet, usdt_dapp_instance, provider);
}