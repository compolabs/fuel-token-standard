contract;
/*   
    ███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
    ██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
    ███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
    ╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
    ███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
    ╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         
*/

use std::{
    address::*,
    auth::{
        AuthError,
        msg_sender,
    },
    identity::{Identity},
    call_frames::{contract_id, msg_asset_id},
    context::{balance_of, msg_amount},
    contract_id::ContractId,
    revert::require,
    storage::*,
    token::*,
};

abi Token {
    // Initialize contract
    #[storage(read, write)]
    fn initialize(config: TokenInitializeConfig, mint_amount: u64, address: Identity);
    // Set mint amount for each address
    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64);
    // Get balance of the contract coins
    fn get_balance() -> u64;
    // Return the mint amount
    #[storage(read)]
    fn get_mint_amount() -> u64;
    // Get balance of a specified token on contract
    fn get_token_balance(asset_id: ContractId) -> u64;
    // Mint token coins
    #[storage(read)]
    fn mint_coins(mint_amount: u64);
    // Burn token coins
    #[storage(read)]
    fn burn_coins(burn_amount: u64);
    // Transfer a contract coins to a given output
    #[storage(read)]
    fn transfer_coins(coins: u64, address: Identity);
    // Transfer a specified token from the contract to a given output
    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, address: Identity);
    // Method called from address to mint coins
    #[storage(read, write)]
    fn mint();    
    // Config of token
    #[storage(read)]
    fn config() -> TokenInitializeConfig;
    // Is user already minted test token
    #[storage(read)]
    fn already_minted(address: Identity) -> bool;
}

const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

pub struct TokenInitializeConfig {
    name: str[32],
    symbol: str[8],
    decimals: u8,
}


storage {
    config: TokenInitializeConfig = TokenInitializeConfig {
        name: "                                ",
        symbol: "        ",
        decimals: 1u8,
    },
    owner: Identity = Identity::Address(Address::from(ZERO_B256)),
    mint_amount: u64 = 0,
    mint_list: StorageMap<Identity, bool> = StorageMap {},
}

enum Error {
    AddressAlreadyMint: (),
    CannotReinitialize: (),
    MintIsClosed: (),
    NotOwner: (),
}


#[storage(read)]
fn validate_owner() {
    let sender = msg_sender().unwrap(); 
    require(storage.owner == sender, Error::NotOwner);
}

impl Token for Contract {
    //////////////////////////////////////
    // Owner methods
    //////////////////////////////////////
    #[storage(read, write)]
    fn initialize(config: TokenInitializeConfig, mint_amount: u64, owner: Identity) {
        require(storage.owner == Identity::Address(Address::from(ZERO_B256)), Error::CannotReinitialize);
        storage.owner = owner;
        storage.mint_amount = mint_amount;
        storage.config = config;
    }

    #[storage(read, write)]
    fn set_mint_amount(mint_amount: u64) {
        validate_owner();
        storage.mint_amount = mint_amount;
    }

    #[storage(read)]
    fn mint_coins(mint_amount: u64) {
        validate_owner();
        mint(mint_amount);
    }

    #[storage(read)]
    fn burn_coins(burn_amount: u64) {
        validate_owner();
        burn(burn_amount);
    }

    #[storage(read)]
    fn transfer_coins(coins: u64, address: Identity) {
        validate_owner();
        transfer(coins, contract_id(), address);
    }

    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, address: Identity) {
        validate_owner();
        transfer(coins, asset_id, address);
    }

    //////////////////////////////////////
    // Mint public method
    //////////////////////////////////////
    #[storage(read, write)]
    fn mint() {
        require(storage.mint_amount > 0, Error::MintIsClosed);

        // Enable a address to mint only once
        let sender = msg_sender().unwrap();
        require(storage.mint_list.get(sender) == false, Error::AddressAlreadyMint);

        storage.mint_list.insert(sender, true);
        mint_to(storage.mint_amount, sender);
    }

    //////////////////////////////////////
    // Read-Only methods
    //////////////////////////////////////
    #[storage(read)]
    fn get_mint_amount() -> u64 {
        storage.mint_amount
    }

    fn get_balance() -> u64 {
        balance_of(contract_id(), contract_id())
    }

    fn get_token_balance(asset_id: ContractId) -> u64 {
        balance_of(asset_id, contract_id())
    }
    #[storage(read)]
    fn config() -> TokenInitializeConfig {
        storage.config
    }

    #[storage(read)]
    fn already_minted(address: Identity) -> bool{
        storage.mint_list.get(address)
    }
}
