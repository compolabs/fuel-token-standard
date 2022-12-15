contract;
use std::{
    auth::{AuthError, msg_sender},
    address::*,
    revert::require,
    context::{balance_of},
    contract_id::ContractId,
    storage::*,
    token::*,
    call_frames::contract_id
};

abi Token {
    // Name of token
    #[storage(read)]
    fn name() -> str[9];
    // Symbol of token
    #[storage(read)]
    fn symbol() -> str[4];
    // decimals of token
    #[storage(read)]
    fn decimals() -> u8;
    // Initialize contract
    #[storage(read, write)]
    fn initialize(mint_amount: u64, address: Address);
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
    fn transfer_coins(coins: u64, address: Address);
    // Transfer a specified token from the contract to a given output
    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, address: Address);
    // Method called from address to mint coins
    #[storage(read, write)]
    fn mint();
}


const ZERO_B256 = 0x0000000000000000000000000000000000000000000000000000000000000000;

storage {
    name: str[9] = "USD Token",
    symbol: str[4] = "USDT",
    decimals: u8 = 6,
    owner: Address = Address { value: ZERO_B256 },
    mint_amount: u64 = 0,
    mint_list: StorageMap<Address, bool> = StorageMap {},
}

enum Error {
    AddressAlreadyMint: (),
    CannotReinitialize: (),
    MintIsClosed: (),
    NotOwner: (),
}

pub fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
       address
    } else {
       revert(0);
    }
}

#[storage(read)]
fn validate_owner() {
    let sender = get_msg_sender_address_or_panic();
    require(storage.owner == sender, Error::NotOwner);
}

impl Token for Contract {
    //////////////////////////////////////
    // Owner methods
    //////////////////////////////////////
    #[storage(read, write)]
    fn initialize(mint_amount: u64,owner: Address) {
        require(storage.owner.into() == ZERO_B256, Error::CannotReinitialize);
        storage.owner = owner;
        storage.mint_amount = mint_amount;
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
    fn transfer_coins(coins: u64, address: Address) {
        validate_owner();
        transfer_to_address(coins, contract_id(), address);
    }

    #[storage(read)]
    fn transfer_token_to_output(coins: u64, asset_id: ContractId, address: Address) {
        validate_owner();
        transfer_to_address(coins, asset_id, address);
    }

    //////////////////////////////////////
    // Mint public method
    //////////////////////////////////////
    #[storage(read, write)]
    fn mint() {
        require(storage.mint_amount > 0, Error::MintIsClosed);

        // Enable a address to mint only once
        let sender = get_msg_sender_address_or_panic();
        require(storage.mint_list.get(sender) == false, Error::AddressAlreadyMint);

        storage.mint_list.insert(sender, true);
        mint_to_address(storage.mint_amount, sender);
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
    fn decimals() -> u8 {
        storage.decimals
    }
    #[storage(read)]
    fn symbol() -> str[4] {
        storage.symbol
    }    
    #[storage(read)]
    fn name() -> str[9] {
        storage.name
    }
}
