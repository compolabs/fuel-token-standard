use dotenv::dotenv;
use fuels::prelude::*;
use rand::prelude::{Rng};

use crate::utils::parse_units;
abigen!(TokenContract, "out/debug/token_contract-abi.json");

#[derive(Debug)]
struct DeployConfig {
    name: String,
    symbol: String,
    decimals: u8,
    mint_amount: u64,
}

const RPC: &str = "node-beta-2.fuel.network";

#[tokio::test]
async fn deploy() {
    let configs: Vec<DeployConfig> = vec![
        DeployConfig {
            name: String::from("Tether"),
            symbol: String::from("USDT"),
            decimals: 6,
            mint_amount: 10000,
        },
        DeployConfig {
            name: String::from("USD Coin"),
            symbol: String::from("USDC"),
            decimals: 6,
            mint_amount: 10000,
        },
        DeployConfig {
            name: String::from("Binance USD"),
            symbol: String::from("BUSD"),
            decimals: 6,
            mint_amount: 10000,
        },
        DeployConfig {
            name: String::from("Bitcoin"),
            symbol: String::from("BTC"),
            decimals: 8,
            mint_amount: 1,
        },
        DeployConfig {
            name: String::from("BNB"),
            symbol: String::from("BNB"),
            decimals: 8,
            mint_amount: 5,
        },
        DeployConfig {
            name: String::from("Uniswap"),
            symbol: String::from("UNI"),
            decimals: 9,
            mint_amount: 1000,
        },
        DeployConfig {
            name: String::from("Chainlink"),
            symbol: String::from("LINK"),
            decimals: 9,
            mint_amount: 1000,
        },
    ];

    for config in configs {
        // println!("{:#?}",config);
        deploy_token_contract(config).await;
    }
}

async fn deploy_token_contract(mut deploy_config: DeployConfig) {
    let provider = match Provider::connect(RPC).await {
        Ok(p) => p,
        Err(error) => panic!("❌ Problem creating provider: {:#?}", error),
    };

    dotenv().ok();
    let secret = match std::env::var("SECRET") {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    deploy_config
        .name
        .push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    deploy_config
        .symbol
        .push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    let token_contract_id = Contract::deploy_with_parameters(
        "out/debug/token_contract.bin",
        &wallet,
        TxParameters::new(Some(1), None, None),
        StorageConfiguration::default(),
        Salt::from(salt),
    )
    .await;
    let token_contract_id = match token_contract_id {
        Ok(id) => id,
        Err(error) => panic!("❌ Problem deploing the dapp: {:#?}", error),
    };

    let instance = TokenContract::new(token_contract_id.clone(), wallet.clone());
    let methods = instance.methods();

    let mint_amount = parse_units(deploy_config.mint_amount, deploy_config.decimals);
    let config: tokencontract_mod::Config = tokencontract_mod::Config {
        name: fuels::core::types::SizedAsciiString::<32>::new(deploy_config.name).unwrap(),
        symbol: fuels::core::types::SizedAsciiString::<8>::new(deploy_config.symbol).unwrap(),
        decimals: deploy_config.decimals,
    };
    let _res = methods
        .initialize(config, mint_amount, Address::from(wallet.address()))
        .tx_params(TxParameters::new(Some(1), Some(1000000), None))
        .call()
        .await;
    println!("{} Initialize\n", if _res.is_ok() { "✅" } else { "❌" });

    let conf = methods.config().simulate().await.unwrap().value;
    println!("ownerAddress: {}", wallet.address());
    println!("name:         {}", conf.name);
    println!("symbol:       {}", conf.symbol);
    println!("decimals:     {}", conf.decimals);
    println!("assetId:      {}", instance.get_contract_id());
    println!("hash:         {}", instance.get_contract_id().hash());
    println!("salt:         {:?}", salt);
    println!("\n");
}
