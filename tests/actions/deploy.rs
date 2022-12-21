use dotenv::dotenv;
use fuels::prelude::*;

use crate::utils::parse_units;
abigen!(TokenContract, "out/debug/token_contract-abi.json");

#[derive(Debug)]
struct DeployConfig {
    name: String,
    symbol: String,
    decimals: u8,
    mint_amount: u64,
    secret: String,
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
            secret: String::from("SECRET_0"),
        },
        DeployConfig {
            name: String::from("USD Coin"),
            symbol: String::from("USDC"),
            decimals: 6,
            mint_amount: 10000,
            secret: String::from("SECRET_1"),
        },
        DeployConfig {
            name: String::from("Binance USD"),
            symbol: String::from("BUSD"),
            decimals: 6,
            mint_amount: 10000,
            secret: String::from("SECRET_2"),
        },
        DeployConfig {
            name: String::from("Bitcoin"),
            symbol: String::from("BTC"),
            decimals: 8,
            mint_amount: 1,
            secret: String::from("SECRET_3"),
        },
        DeployConfig {
            name: String::from("BNB"),
            symbol: String::from("BNB"),
            decimals: 8,
            mint_amount: 5,
            secret: String::from("SECRET_4"),
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
    let secret = match std::env::var(deploy_config.secret) {
        Ok(s) => s,
        Err(error) => panic!("❌ Cannot find .env file: {:#?}", error),
    };

    let wallet = WalletUnlocked::new_from_private_key(secret.parse().unwrap(), Some(provider));

    let token_contract_id = Contract::deploy(
        "out/debug/token_contract.bin",
        &wallet,
        TxParameters::new(Some(1), None, None),
        StorageConfiguration::default(),
    )
    .await;
    let token_contract_id = match token_contract_id {
        Ok(id) => id,
        Err(error) => panic!("❌ Problem deploing the dapp: {:#?}", error),
    };

    let instance = TokenContract::new(token_contract_id.clone(), wallet.clone());
    let methods = instance.methods();

    deploy_config.name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    deploy_config.symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());
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

    // print!("\nOwner secret  {}", deploy_config.secret.to_string());
    let conf = methods.config().simulate().await.unwrap().value;
    print!("\nownerAddress: {}", wallet.address());
    print!("\nname: {}", conf.name);
    print!("\nsymbol: {}", conf.symbol);
    print!("\ndecimals: {}", conf.decimals);
    println!("\nassetId: {}", instance.get_contract_id());
    println!("\nhash: {}", instance.get_contract_id().hash());
}
