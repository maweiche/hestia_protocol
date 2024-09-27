# Hestia - Take your Restaurant on-chain

A Solana program built for to take small/medium sized food-service operations on-chain without any disruptions to current operations.

Below you can find instructions to running the program locally followed by a deeper dive into the program's structure and details.

## Program ID
| Cluster      | Program Id |
| :---        |    :----:   |
| **Localnet**     | `BfKK2fRqZKyX2qce7UEkKntUCK9BMQR1ozgmitvPQtD2` |
| **Devnet**  | `BfKK2fRqZKyX2qce7UEkKntUCK9BMQR1ozgmitvPQtD2` |
| **Mainnet**  | ``  |

## Description

The Hestia Protocol is structured to set up an umbrella of inter-connected Solana accounts for each restaurant to operate. The Protocol currently covers the following area of operations:
- In-Store/Mobile P.O.S. (Place/Update/Cancel Order, customers can pay crypto or card)
- Inventory (Standard C.R.U.D Operations)
- Customer Mgmt (Creates a unique Customer account for each Restaurant)
- Customer Rewards (Create/Distribute NFTs as Customer Rewards that can be redeemed for free items)
- Employee Mgmt (Add/Promote/Remove Employees from your Restaurant)
- Menu Mgmt (Standard C.R.U.D Operations for each Restaurant's Menu)

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Anchor](https://www.anchor-lang.com/)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Solana Wallet](https://docs.solanalabs.com/cli/wallets/paper)

***For testing purposes it is advised to use at least three developer wallets (Admin, Artist, User)***

> If you don't have Anchor installed, you can use [Solana Playground](https://beta.solpg.io/) to build and deploy.


### Installing

To begin, clone the repo and install the necessary dependencies.

```
git clone https://github.com/maweiche/hestia_protocol.git
cd hestia_protocol
npm install
```

Inside of the `Anchor.toml` at the root level edit the cluster you want to test on and the following path to your local Wallet (**Admin Wallet**).

```rust
[provider]
cluster = "localnet"
wallet = "/Users/matt/.config/solana/id.json"
```

Next, set your local Wallet address to the `admin_wallet` in `programs/hestia_protocol/src/constant.rs`. This will be the Wallet to perform **Admin** actions within the program.

```rust
pub mod admin_wallet {
    use super::*;
    declare_id!("6KuX26FZqzqpsHDLfkXoBXbQRPEDEbstqNiPBKHNJQ9e");
}
```

### Building and Deploying program

To build and deploy on Solana's Localnet execute the following commands:

1. Make sure your Solana CLI cluster localnet by running: `solana config get`

```
matt@Matts-MBP hestia_protocol % solana config get                
Config File: /Users/matt/.config/solana/cli/config.yml
RPC URL: http://localhost:8899 
WebSocket URL: ws://localhost:8900/ (computed)
Keypair Path: /Users/matt/.config/solana/id.json 
Commitment: confirmed 
```

If you you need to set your Solana CLI cluster to localhost run: `solana config set --url localhost`

2. Start up `solana-test-validator` on a separate tab in your terminal, this will need to be operating while deploying and testing the program.

3. Next, build and deploy the program then grab the **Program Id** returned: 

```
anchor build
anchor deploy

Deploying cluster: http://localhost:8899
Upgrade authority: /Users/matt/.config/solana/id.json
Deploying program "hestia_protocol"...
Program path: /Users/matt/Desktop/AiSol/hestia_protocol/target/deploy/hestia_protocol.so...
Program Id: 4Fj9kuGYLye3pwCBYaXbuzocEy22gPWT5TcJVJ6JauUt

Deploy success
```
4. Execute `anchor keys sync`

5. Build and deploy the program one more time to register the deployed **Program Id** within the program itself.

```
anchor build
anchor deploy
```

### Testing program

The program comes with tests already designed to demo Restaurant and Customer actions. 

It is also recomended to create a directory at the root level and name it `test-wallet` and place two Keypairs to use in addition to your local Solana CLI wallet for testing.

🚨 **`test-wallet` is already included in the `.gitignore` if you name the folder something else then make sure to add the folder name to the `.gitignore`**

The current `Keypair` function utilizes a `Uint8array`, if using a Private Key exported from a Wallet like Phantom then change it to:

```ts
const buyer = Keypair.fromSecretKey(base58.decode(buyer_keypair))
```

To execute the tests, with `solana-test-validator` running, use the command:

```
anchor test
```

**🚨 Important Notes**
- The test file should be ran in complete totality just once. After the first successful execution the following tests can be skipped:
```ts
- "Protocol is initialized"
- "Protocol is toggled to lock"
- "Protocol is toggled to unlock"
```

## Program Structure and Details

The program takes on the following Structure:

```ssh 
hestia_protocol (owner: Admin)
|
|-> Restaurant (owner: restaurant_admin)
    |
    |-> Employee (owner: Restaurant, authority: restaurant_admin)
    |-> Inventory (owner: Restaurant, authority: restaurant_admin)
    |-> Menu (owner: Restaurant, authority: restaurant_admin)
    |-> Customer (owner: Restaurant, authority: customer)
    |-> CustomerRewards (owner: Restaurant/Customer, authority: restaurant_admin/customer)
```

## Down-the-Road Ideas
    1. Employee NFT instead of PDA, allow for employee to verify employment by scanning at other locations -> discounts etc.

    2. Need a better inventory formula for tracking on menuitem, rn it is just sku, but need sku/amount -> ingredient pda? 

    3. Set tax rates on Restaurant state & Create additional account to reconcile taxes? 

    4. Payroll from Restaurant, daily settlement?

## Help

Please reach out on Telegram or Twitter.

## Authors

Contributors names and contact info

[Matt Weichel](https://github.com/maweiche)

## License

The Vision - Sol Factory Solana Program is licensed under the [MIT License](https://github.com/playground-solana/vision_program/LICENSE).
