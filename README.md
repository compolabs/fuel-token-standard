# Fuel network Fungible token standard: FRC-20
## INTRODUCTION

### What is a Token?

Tokens can represent virtually anything in Fuel ⚡️:

- reputation points in an online platform
- skills of a character in a game
- lottery tickets
- financial assets like a share in a company
- a fiat currency like USD
- an ounce of gold
- and more...

Such a powerful feature of Fuel must be handled by a robust standard, right? That's exactly where the FRC-20 plays its role! This standard allows developers to build token applications that are interoperable with other products and services.

### What is ERC-20?

The FRC-20 introduces a standard for Fungible Tokens, in other words, they have a property that makes each Token be exactly the same (in type and value) as another Token. For example, a FRC-20 Token acts just like the ETH, meaning that 1 Token is and will always be equal to all the other Tokens.

## BODY

The FRC-20, proposed by the SWAY GANG TEAM on December 2022, is a Token Standard that implements an API for tokens within Smart Contracts.

Example functionalities FRC-20 provides:

- transfer tokens from one account to another
- get the current token balance of an account
- get the total supply of the token available on the network *(soon)*
- approve whether an amount of token from an account can be spent by a third-party account

If a Smart Contract implements the following methods and events it can be called an FRC-20 Contract and, once deployed, it will be responsible to keep track of the created tokens on Fuel.

Hello, fuel!!
Continuing [the topic](https://forum.fuel.network/t/which-token-contract-i-should-to-use/373), I want to tell you about how to deploy and use your own token in your dapp

# How to issue my own token on the fuel testnet?

# How to issue my own token on the fuel testnet using Rust SDK?

## Intro
In the continuation of [this topic](https://forum.fuel.network/t/which-token-contract-i-should-to-use/373), I wrote a tutorial on how to work with tokens. This tutorial walks you through the process of issuing your own token on the Fuel testnet using Rust SDK.

## Deploy

### Clone repository
This step involves cloning the fuel-token-standard repository from GitHub onto your local computer. To do this, you will need to use the `git clone` command.

`git clone https://github.com/sway-gang/fuel-token-standard`

After that let's go to the project folder

`cd fuel-token-standard`

### Setup env
In this step, you will set up the environment for the fuel-token-standard repository. Here we should use  `forc-wallet`.  To create this, you will need to follow the instructions provided in the `forc-wallet` [repository on GitHub](https://github.com/FuelLabs/forc-wallet). 

After creating a wallet, you can see a list of your accounts by using this command

`forc-wallet list` 

You also need to visit the Fuel faucet  [to get some testnet FUEL tokens](https://faucet-beta-2.fuel.network/) , which you will use to deploy and interact with your token contract.


Here will involve creating a `.env` file and adding your wallet secret to it. To do this, you can rename the template:

`mv ./.env.example ./.env`

And insert your secret inside the `.env` file
```
SECRET=YOUR_SECRET_HERE
```


### Build and deploy

> In this section I will explain in detail how to deploy, initialize, mint and transfer tokens. If you just want to deploy tokens in one click, then skip this section and move on to the next

In this step, you will build and deploy your token contract using the `forc` command-line tool. This involves compiling the contract code, generating the required bytecode, and deploying the contract to the Fuel testnet.

To build the contract, you will use the `forc build` command, which will compile the contract code and generate the bytecode required for deployment.

```
> forc build
  Compiled library "core".
  Compiled library "std".
  Compiled contract "token_contract".
  Bytecode size is 13912 bytes.
```



To deploy the contract, you will use the `forc deploy` command, which will send the deployment transaction to the Fuel testnet. 

`forc deploy --url http://node-beta-2.fuel.network/graphql --gas-price 1  `

Output:
```
  Compiled library "core".
  Compiled library "std".
  Compiled contract "token_contract".
  Bytecode size is 13912 bytes.
Contract id: <ACCOUNT ID>
Please provide the address of the wallet you are going to sign this transaction with:fuel1s0ul05vsv84ltlxfn7fwmv0765ghah4nm5zj84z6zwy9mcutnz6q97zrcl
Transaction id to sign: 4232f9c81c5104f3f9a81a120d0bec084c8fa8be40ad6397fd7f2aa79bd8af91
Please provide the signature: <YOUR SIGNATURE>
```
Before the deployment can be completed, you will need to sign the transaction with your wallet. To do this, you will need to use the `forc-wallet` command-line tool. First, you will need to open another terminal and use the `forc-wallet list` command to view the available wallets on your machine.  

Next, you will need to use the `forc-wallet sign` command to sign the transaction with the desired wallet. The command requires you to specify the transaction ID and the index of the wallet you want to use.

`forc-wallet sign <TRANSACTION ID> <WALLET INDEX>`

```
Please enter your password to decrypt initialized wallet's phrases: 
Signature: 8d9c894fcbb5ae3f1a0318cca5c783e9a67b150ca953350023c673cb640171ad12e68716d17cf7bfb2e201dd72c506ede78ccf762bed621fa37275863a2010c3
```

Finally, you will need to enter the signature in the first terminal to complete the deployment. If the deployment is successful, the output will include the contract ID and the block number in which the contract was deployed.

```
Waiting 1s before retrying    
contract 777923117c7772c0680806d2a0d3a0eb5e654fa65e48d8de85516f6f85ba4887 deployed in block 0xcc6320caa393126b98bdbd0d90fba9cd659a36546b567f7d90ba8b591020133d
```

## Initialize
The `initialize` test case initializes the token contract by setting its name, symbol, and number of decimals. 

To run the test case, update the `TOKEN_ADDRESS` constant in the `tests/testnet_tests/initialize.rs` file with the address of your token contract, then run the following command:

```
cargo test --package tests --test tests -- testnet_tests::initialize::initialize --exact --nocapture 
```

Example output:



```
running 1 test
👛 Account address     @ fuel1gzv3z02hz863dhtxzhz2d30jh62verdx7lfrzxkq9txxr3jgx4qsqgstc4
🗞 Token address   @ fuel1waujxytuwaevq6qgqmf2p5aqad0x2naxteyd3h5929hklpd6fzrsyjw5jy      
✅ Initialize
Decimals: 9
Symbol:  BTC
Mint amount 1000 BTC   
test testnet_tests::initialize::initialize ... ok
```


## Mint

The `mint` test case mints tokens to the wallet specified in the test configuration.

To run the test case, update the `TOKEN_ADDRESS` constant in the `tests/testnet_tests/mint.rs` file with the address of your token contract, then run the following command:

```
cargo test --package tests --test tests -- testnet_tests::mint::mint --exact --nocapture 
```

Example output:

```
running 1 test
👛 Account address     @ fuel1gzv3z02hz863dhtxzhz2d30jh62verdx7lfrzxkq9txxr3jgx4qsqgstc4
🗞 Token address   @ fuel1waujxytuwaevq6qgqmf2p5aqad0x2naxteyd3h5929hklpd6fzrsyjw5jy
Decimals: 1
Symbol: BBC     
Mint amount: 1000 BBC     
Wallet balance: 1000 BBC     
✅ Mint
Wallet balance: 1000 BBC     
test testnet_tests::mint::mint ... ok
```
## Transfer

The `transfer` test case transfers a specified amount of tokens from the wallet specified in the test configuration to the recipient specified in the test configuration.

To run the test case, update the `TOKEN_ADDRESS`, `TRANSFER_AMOUNT`, and `RECIPIENT_ADDRESS` constants in the `tests/testnet_tests/transfer.rs` file with the address of your token contract, the amount of tokens to transfer (without decimals), and the recipient address, respectively, then run the following command:

```
cargo test --package tests --test tests -- testnet_tests::transfer::transfer --exact --nocapture 
```

Example output:

```
running 1 test
👛 Account address     @ fuel1gzv3z02hz863dhtxzhz2d30jh62verdx7lfrzxkq9txxr3jgx4qsqgstc4
🗞  Token address   @ fuel1waujxytuwaevq6qgqmf2p5aqad0x2naxteyd3h5929hklpd6fzrsyjw5jy
Decimals: 1
Symbol: BBC     
Wallet balance: 1000 BBC     
Wallet balance: 990 BBC     
Recipient balance: 10 BBC     
test testnet_tests::transfer::transfer ... ok
```


## Deploy multiple tokens in one click

To deploy multiple tokens in one click, you will need to use the `tests/testnet_tests/deploy.rs` script. In this script, you can specify an array of token configurations, each containing the name, symbol, decimals, and mint amount for the desired tokens.

To specify the tokens you want to deploy, modify the `configs` array in the script. For example:

```
 // YOUR TOKENS ARRAY HERE
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

...

   ]
```

Once you have specified the desired tokens, you can run the script with the following command:

```
cargo test --package tests --test tests -- testnet_tests::deploy::deploy --exact --nocapture 
```

This will build and deploy the specified tokens. The output of the command will show the initialization details for each token, as well as any additional information relevant to the deployment process.

Example output:

```
running 1 test
✅ Initialize

ownerAddress: fuel1gzv3z02hz863dhtxzhz2d30jh62verdx7lfrzxkq9txxr3jgx4qsqgstc4
name:         Tether                          
symbol:       USDT    
decimals:     6
assetId:      fuel1v4eezwvca24cn9xkp84kf26wm06cvfwef0zgsyk7mfvpqw3tm2qqf0rqha
hash:         6573913998eaab8994d609eb64ab4edbf58625d94bc48812deda58103a2bda80
salt:         [229, 154, 86, 240, 24, 119, 67, 186, 120, 209, 57, 181, 200, 35, 138, 119, 12, 183, 120, 179, 122, 172, 253, 77, 193, 37, 107, 157, 45, 225, 60, 163]

...

test testnet_tests::deploy::deploy ... ok
```

### Can I just use your tokens?

If you don't want to spend your time deploying your own contract here is the config to the token that already exists in testnet:

https://github.com/sway-gang/sway-lend/blob/master/frontend/src/constants/tokens.json

And here is the link where you can mint them: https://app.swaylend.com/#/faucet.

## Conclusion
In this tutorial, you have learned how to build and deploy a token contract using the `forc` command-line tool. You have also seen how to perform various actions with the token, such as initialize, mint, transfer, and deploy multiple tokens in one click. By following the steps outlined in this tutorial, you can quickly and easily create and manage your own token on the Fuel network.

## Resources

https://fuellabs.github.io/fuel-docs/master/index.html

https://fuellabs.github.io/sway/v0.32.2/book/

https://fuellabs.github.io/fuels-rs/v0.33.0/

https://github.com/FuelLabs/swayswap

https://forum.fuel.network/t/src-20-fungible-token-standard/186
