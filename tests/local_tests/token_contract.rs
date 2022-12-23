use crate::utils::{local_tests_utils::{abi_calls::*, setup_utils::setup}, number_utils::*};
use fuels::prelude::*;

#[tokio::test]
async fn token_contract() {
    ////////////////////////////////////////////////////////
    //  Setup contracts and wallets
    ////////////////////////////////////////////////////////
    let (owner_token_istance, wallets) = setup().await;

    let config = get_config(&owner_token_istance).await.unwrap().value;
    let symbol = config.symbol;
    let name = config.name;
    let decimals = config.decimals;

    println!(
        " 🪙  Token contract id: {}",
        owner_token_istance.get_contract_id()
    );
    println!(" 👮 Wallet owner     : {}", wallets.wallet_owner.address());
    println!("\n ℹ️  Name: {name}\n ℹ️  Symbol: {symbol}\n ℹ️  Decimals: {decimals}");
    let token_mint_amount = parse_units(10000, decimals); // Get the contract ID and a handle to it
    let wallet_token_amount = parse_units(1000, decimals); // Amount of tokens given to the wallet
    let asset_id = AssetId::from(*owner_token_istance.get_contract_id().hash());

    // Initialize contract
    initialize(
        &owner_token_istance,
        "Big Block Coin",
        "BBC",
        9,
        token_mint_amount,
        Address::from(wallets.wallet_owner.address()),
    )
    .await
    .unwrap();
    let mint_amount = get_mint_amount(&owner_token_istance).await.unwrap().value;
    println!(
        "\n✅ Initialize\n   Mint amount: {} {symbol}\n   Expected mint amount: {} {symbol}",
        format_units(mint_amount, decimals),
        format_units(token_mint_amount, decimals)
    );

    ////////////////////////////////////////////////////////
    // Test Token Contract
    ////////////////////////////////////////////////////////

    // Contract can be initialized only once
    let is_error = initialize(
        &owner_token_istance,
        "Big Block Coin",
        "BBC",
        9,
        token_mint_amount,
        Address::from(wallets.wallet_owner.address()),
    )
    .await
    .is_err();
    assert!(is_error);
    println!("\n✅ Contract can be initialized only once");

    // Verify the mint amount
    let mint_amount_contract = get_mint_amount(&owner_token_istance).await.unwrap();
    assert_eq!(mint_amount_contract.value, token_mint_amount);
    println!(
        "\n✅ Verify the mint amount\n   get_mint_amount result: {} {symbol}\n   token_mint_amount: {} {symbol}",
        format_units(mint_amount_contract.value, decimals), format_units(token_mint_amount, decimals)
    );

    // Verify update mint amount
    set_mint_amount(&owner_token_istance, parse_units(1, decimals))
        .await
        .unwrap();
    let mint_amount_contract = get_mint_amount(&owner_token_istance).await.unwrap();
    assert_eq!(mint_amount_contract.value, parse_units(1, decimals));
    println!(
        "\n✅ Verify update mint amount\n   get_mint_amount result: {} {symbol}\n   token_mint_amount: {} {symbol}",
        format_units(mint_amount_contract.value, decimals), 1
    );

    // Update mint amount to the original value
    set_mint_amount(&owner_token_istance, token_mint_amount)
        .await
        .unwrap();
    println!(
        "\n✅ Update mint amount to the original value: {} {symbol}",
        format_units(token_mint_amount, decimals)
    );

    // Mint some tokens
    mint_coins(&owner_token_istance, token_mint_amount)
        .await
        .unwrap();

    // Check the balance of the contract of its own asset
    let result = get_balance(&owner_token_istance).await.unwrap();
    assert_eq!(result.value, token_mint_amount);
    println!(
        "\n✅ Mint coins\n   get_balance result: {} {symbol}\n   token_mint_amount: {} {symbol}",
        format_units(token_mint_amount, decimals),
        format_units(result.value, decimals)
    );

    // Transfer tokens to the wallet
    let address = Address::from(wallets.wallet_owner.address());
    transfer_coins(&owner_token_istance, wallet_token_amount, address.clone())
        .await
        .unwrap();
    // Check the balance of the contract of its own asset
    let result = get_balance(&owner_token_istance).await.unwrap();
    let contract_balance = token_mint_amount - wallet_token_amount;
    let wallet_balance = wallets
        .wallet_owner
        .get_asset_balance(&asset_id)
        .await
        .unwrap();
    assert_eq!(result.value, contract_balance);
    println!(
        "\n✅ Transfer {} {symbol} to the wallet\n   Expected contract balance: {} {symbol}\n   Contract balance: {} {symbol}\n   Wallet balance: {} {symbol}",
        format_units(wallet_token_amount, decimals),
        format_units(contract_balance, decimals),
        format_units(result.value, decimals),
        format_units(wallet_balance, decimals),
    );

    // Burn all minted coins
    burn_coins(&owner_token_istance, contract_balance)
        .await
        .unwrap();

    // Check the balance of the contract of its own asset
    let result = get_balance(&owner_token_istance).await.unwrap();
    assert_eq!(result.value, 0);
    println!(
        "\n✅ Burn all minted coins\n   Minted amount: {} {symbol}",
        format_units(result.value, decimals),
    );
}
