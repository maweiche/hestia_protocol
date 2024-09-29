# Hestia - Take your Restaurant on-chain

## Overview

Hestia Protocol is a comprehensive restaurant management system built on the Solana blockchain. It provides a robust set of tools for restaurant owners, employees, and customers, facilitating efficient operations, inventory management, menu creation, order processing, and customer reward programs.

## Program ID
| Cluster      | Program Id |
| :---        |    :----:   |
| **Localnet**     | `BfKK2fRqZKyX2qce7UEkKntUCK9BMQR1ozgmitvPQtD2` |
| **Devnet**  | `BfKK2fRqZKyX2qce7UEkKntUCK9BMQR1ozgmitvPQtD2` |
| **Mainnet**  | ``  |

## Table of Contents

1. [Program Structure](#program-structure)
2. [Key Components](#key-components)
3. [Core Functionalities](#core-functionalities)
4. [Detailed Component Descriptions](#detailed-component-descriptions)
5. [Usage Examples](#usage-examples)
6. [Security Considerations](#security-considerations)
7. [Future Enhancements](#future-enhancements)
8. [Clone & Run Locally](#getting-started)

## Program Structure

The Hestia Protocol is organized into several modules:

- `lib.rs`: The main entry point of the program, defining all instruction handlers.
- `state.rs`: Defines the structure of all accounts used in the protocol.
- `instructions/`: Contains individual instruction logic for each operation.
- `errors.rs`: Defines custom error types for the protocol.
- `constants.rs`: Stores constant values used throughout the program.

## Key Components

1. Protocol Management
2. Restaurant Management
3. Employee Management
4. Inventory Management
5. Menu Management
6. Order Processing
7. Customer Rewards System

## Core Functionalities

### 1. Protocol Management

- Initialize protocol
- Toggle protocol lock status
- Add/Remove protocol admins

### 2. Restaurant Management

- Initialize restaurant
- Manage restaurant details

### 3. Employee Management

- Add, remove, and promote employees
- Assign roles and permissions

### 4. Inventory Management

- Add, update, and remove inventory items
- Track stock levels

### 5. Menu Management

- Create and modify menu items
- Link menu items to inventory

### 6. Order Processing

- Create and manage customer orders
- Update order status

### 7. Customer Rewards System

- Create and manage reward programs
- Issue and redeem reward vouchers

## Detailed Component Descriptions

### Protocol Management

The protocol is initialized with a global state and admin account. The protocol utilizes Metaplex's `mpl_core` , more specifically their `OracleValidation`, to maintain a switch that can effectively "turn on/off" the protocol if a security breach occurs.

```rust
fn initialize_protocol(&mut self, bump: u8) -> Result<()> { 
    self.protocol.set_inner(Protocol {
        validation: OracleValidation::V1 {
            create: ExternalValidationResult::Approved,
            transfer: ExternalValidationResult::Approved,
            burn: ExternalValidationResult::Approved,
            update: ExternalValidationResult::Approved,
        },
        bump,
    });

    Ok(())
}

// example of toggled off
OracleValidation::V1 {
    create: ExternalValidationResult::Rejected,
    transfer: ExternalValidationResult::Rejected,
    burn: ExternalValidationResult::Rejected,
    update: ExternalValidationResult::Rejected,
}
```

Admins can be added or removed:

```rust
pub struct AdminProfile {
    pub username: String,
    pub creation_time: i64,
    pub bump: u8,
}

pub fn protocol_add_admin(ctx: Context<AdminInit>, username: String) -> Result<()> {
    instructions::initialize_admin::handler(ctx, username)
}

pub fn protocol_remove_admin(ctx: Context<AdminRemove>) -> Result<()> {
    instructions::remove_admin::handler(ctx)
}
```

### Restaurant Management

Restaurants are initialized with specific details:

```rust
pub struct Restaurant {
    pub restaurant_type: RestaurantType,
    pub owner: Pubkey,
    pub name: String,
    pub symbol: String,
    pub currency: Pubkey,
    pub url: String,
    pub customer_count: u64,
    pub bump: u8,
}

pub enum RestaurantType {
    Foodtruck,
    Cafe,
    Restaurant,
}

pub fn restaurant_initialize(ctx: Context<CreateRestaurant>, args: CreateRestaurantArgs) -> Result<()> {
    instructions::initialize_restaurant::handler(ctx, args)
}
```

### Employee Management

Employees can be added, removed, or promoted:

```rust
pub struct Employee {
    pub wallet: Pubkey,
    pub restaurant: Pubkey,
    pub employee_type: EmployeeType,
    pub username: String,
    pub initialized: bool,
    pub bump: u8,
}

pub enum EmployeeType {
    TeamMember,
    TeamLeader,
    Manager,
    Director,
}

pub fn restaurant_add_employee(ctx: Context<AddEmployee>, args: AddEmployeeArgs) -> Result<()> {
    instructions::add_employee::handler(ctx, args)
}

pub fn restaurant_remove_employee(ctx: Context<RemoveEmployee>, args: RemoveEmployeeArgs) -> Result<()> {
    instructions::remove_employee::handler(ctx, args)
}

pub fn restaurant_promote_employee(ctx: Context<PromoteEmployee>, args: PromoteEmployeeArgs) -> Result<()> {
    instructions::promote_employee::handler(ctx, args)
}
```

### Inventory Management

Inventory items can be added, updated, or removed. Inventory account attributes are in-line with standard records, using a sku and extending an `InventoryCategoryType` to allow for faster sorting on the front-end and clearer detailed inventory reports.

```rust
pub struct InventoryItem {
    pub sku: String,
    pub category: InventoryCategoryType,
    pub name: String,
    pub price: u64,
    pub stock: u64,
    pub last_order: i64,
    pub initialized: bool,
    pub bump: u8,
}

pub enum InventoryCategoryType {
    PaperGoods,
    CleaningSupplies,
    Food,
    Beverages,
    Alcohol,
    Equipment,
    Uniform,
    Marketing,
    Other,
}

pub fn restaurant_add_inventory_item(ctx: Context<ManageInventory>, args: InventoryArgs) -> Result<()> {
    instructions::add_inventory::handler(ctx, args)
}

pub fn restaurant_remove_inventory_item(ctx: Context<RemoveInventory>, args: RemoveInventoryArgs) -> Result<()> {
    instructions::remove_inventory::handler(ctx, args)
}
```

### Menu Management

Menu items canbe added, updated, or toggled active/inactive. The `MenuItem`, similar to the `InventoryItem`, is maintained by a sku and `MenuCategoryType`. It's program account also includes an `IngredientList` that references inventory items and amounts to allow for a detailed description on the `MenuItem`.

```rust
pub struct MenuItem {
    pub sku: String,
    pub category: MenuCategoryType,
    pub name: String,
    pub price: u64,
    pub description: String,
    pub active: bool,
    pub bump: u8,
}

pub struct IngredientList {
    pub menu_item: Pubkey,
    pub ingredients: Vec<(Pubkey, u64)>, // (InventoryItem pubkey, quantity)
    pub bump: u8,
}

pub fn restaurant_add_menu_item(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    instructions::add_menu_item::add_menu_item_handler(ctx, args)
}

pub fn restaurant_update_menu_item(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    instructions::add_menu_item::update_menu_item_handler(ctx, args)
}

pub fn restaurant_toggle_menu_item(ctx: Context<ToggleMenuItem>, args: ToggleMenuItemArgs) -> Result<()> {
    instructions::toggle_menu_item::handler(ctx, args)
}
```

### Order Processing

Customer Orders can be initiated by the Customer (mobile order) or by a Restaurant Employee (in-store order). The customer can pay via stripe/credit-card or crypto and for each dollar spent they earn 10 points that can be used toward reward-vouchers/free-items. Orders maintain a `status` that begins as `pending` when created and can be `cancelled` by the customer or an employee. This status is then set to `completed` when the order (food/drink etc.) is made, and `finalized` when handed over to the customer, both actions executed by an employee account.

```rust
pub struct CustomerOrder {
    pub order_id: u64,
    pub customer: Pubkey,
    pub items: Vec<u64>,
    pub total: u64,
    pub status: StatusType,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub bump: u8,
}

pub enum StatusType {
    Pending,
    Completed,
    Finalized,
    Cancelled,
}

pub fn restaurant_add_order(ctx: Context<AddCustomerOrder>, args: CustomerOrderArgs) -> Result<()> {
    instructions::add_order::handler(ctx, args)
}

pub fn restaurant_update_order(ctx: Context<UpdateCustomerOrder>, args: UpdateOrderArgs) -> Result<()> {
    instructions::update_order::handler(ctx, args)
}

pub fn restaurant_cancel_order(ctx: Context<CancelCustomerOrder>, args: CancelOrderArgs) -> Result<()> {
    instructions::cancel_order::handler(ctx, args)
}
```

### Customer Rewards System

`Reward` accounts are first created referencing an item specific to the restaurant by `sku`, from here, the restaurant can then `add_reward` and set the parameters for it seen below. 

The `RewardVoucher`'s are generated as Metaplex's Core NFT's using the `mpl_core` program. These rewards can then be purchased by customer's, using the points gained from previous purchases, and redeemed for free items during the order in which the Core NFT is effectively burned. 

When a rewards total alottment is reached it is then transformed into a `CompletedRewardVoucher` version for reference.

```rust
pub struct Reward {
    pub category: MenuCategoryType,
    pub restaurant: Pubkey,
    pub reward_points: u64,
    pub reward_item: Pubkey,
    pub bump: u8,
}

pub struct RewardVoucher {
    pub id: u64,
    pub item_sku: u64,
    pub reward: Pubkey,
    pub restaurant: Pubkey,
    pub category: MenuCategoryType,
    pub share: u16,
    pub share_sold: u16,
    pub price: u64,
    pub starting_time: i64,
    pub bump: u8,
}

pub struct CompletedRewardVoucher {
    pub id: u64,
    pub reward: Pubkey,
    pub category: MenuCategoryType,
    pub share: u16,
    pub price: u64,
    pub bump: u8,
}

pub fn restaurant_create_reward(ctx: Context<CreateReward>, args: CreateRewardArgs) -> Result<()> {
    instructions::create_reward::handler(ctx, args)
}

pub fn restaurant_add_reward(ctx: Context<AddRewardVoucher>, args: AddRewardVoucherArgs) -> Result<()> {
    instructions::add_reward::handler(ctx, args)
}

pub fn customer_buy_reward(ctx: Context<BuyRewardVoucher>, uri: String) -> Result<()> {
    instructions::buy_reward_voucher::handler(ctx, uri)
}
```

## Security Considerations

- Admin access is strictly controlled and can only be modified by existing admins
- Employee permissions are role-based to ensure proper access control
- All critical operations require appropriate authorization checks

## Future Enhhancements

- Allow Restaurants to update geo-location and "open/closed" toggle
- Integration with point-of-sales UI and back-of-house order screen UI
- advanced analytics for restaurant performance (end-of-month reports, hourly performance, food cost report, customer details etc.)
- customer feedback and rating system
- multi-restaurant chain management features
- Employee NFT instead of PDA, allow for employee to verify employment by scanning at other locations -> discounts etc.
- Set tax rates on Restaurant state & Create additional account to reconcile taxes? 
- Payroll from Restaurant, daily settlement?

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

## Help

Please reach out on Telegram or Twitter.

## Authors

Contributors names and contact info

[Matt Weichel](https://github.com/maweiche)