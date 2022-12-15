use dotenv::dotenv;
use fuels::prelude::*;

const RPC: &str = "http://node-beta-1.fuel.network/graphql";

#[tokio::test]
async fn deploy_token_contract() {
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

    let token_contract_id = Contract::deploy(
        "out/debug/token_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/token_contract-storage_slots.json".to_string(),
        )),
    )
    .await;
    let token_contract_id = match token_contract_id {
        Ok(id) => id,
        Err(error) => panic!("❌ Problem deploing the dapp: {:#?}", error),
    };

    println!("✅ Contract deployed @ {token_contract_id}");
}
