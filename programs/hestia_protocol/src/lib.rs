use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BfKK2fRqZKyX2qce7UEkKntUCK9BMQR1ozgmitvPQtD2");

#[program]
pub mod hestia_protocol {
    use super::*;

    /// Protocol Management Functions
    /// These functions handle the core protocol operations and administration

    /// Initialize the protocol and set up the global state with an admin
    pub fn protocol_init(ctx: Context<ProtocolInit>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx)
    }

    /// Toggle protocol lock status (emergency measure)
    pub fn protocol_toggle(ctx: Context<ProtocolToggle>) -> Result<()> {
        instructions::toggle_protocol::handler(ctx)
    }

    /// Add a new protocol admin (can only be called by a current admin)
    pub fn protocol_add_admin(ctx: Context<AdminInit>, username: String) -> Result<()> {
        instructions::initialize_admin::handler(ctx, username)
    }

    /// Remove a protocol admin (can only be called by a current admin)
    pub fn protocol_remove_admin(ctx: Context<AdminRemove>) -> Result<()> {
        instructions::remove_admin::handler(ctx)
    }

    /// Restaurant Management Functions
    /// These functions handle restaurant-specific operations

    /// Initialize a restaurant in the Hestia protocol
    pub fn restaurant_initialize(ctx: Context<CreateRestaurant>, args: CreateRestaurantArgs) -> Result<()> {
        instructions::initialize_restaurant::handler(ctx, args)
    }

    /// Employee Management

    /// Add a restaurant employee
    pub fn restaurant_add_employee(ctx: Context<AddEmployee>, args: AddEmployeeArgs) -> Result<()> {
        instructions::add_employee::handler(ctx, args)
    }

    /// Remove a restaurant employee
    pub fn restaurant_remove_employee(ctx: Context<RemoveEmployee>, args: RemoveEmployeeArgs) -> Result<()> {
        instructions::remove_employee::handler(ctx, args)
    }

    /// Promote a restaurant employee
    pub fn restaurant_promote_employee(ctx: Context<PromoteEmployee>, args: PromoteEmployeeArgs) -> Result<()> {
        instructions::promote_employee::handler(ctx, args)
    }

    /// Inventory Management

    /// Add or update an inventory item
    pub fn restaurant_add_inventory_item(ctx: Context<ManageInventory>, args: InventoryArgs) -> Result<()> {
        instructions::add_inventory::handler(ctx, args)
    }

    /// Remove an inventory item
    pub fn restaurant_remove_inventory_item(ctx: Context<RemoveInventory>, args: RemoveInventoryArgs) -> Result<()> {
        instructions::remove_inventory::handler(ctx, args)
    }

    /// Menu Management

    /// Add a menu item
    pub fn restaurant_add_menu_item(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
        instructions::add_menu_item::add_menu_item_handler(ctx, args)
    }

    /// Update a menu item
    pub fn restaurant_update_menu_item(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
        instructions::add_menu_item::update_menu_item_handler(ctx, args)
    }

    /// Toggle a menu item's availability
    pub fn restaurant_toggle_menu_item(ctx: Context<ToggleMenuItem>, args: ToggleMenuItemArgs) -> Result<()> {
        instructions::toggle_menu_item::handler(ctx, args)
    }

    /// Reward Management

    /// Create a reward account
    pub fn restaurant_create_reward(ctx: Context<CreateReward>, args: CreateRewardArgs) -> Result<()> {
        instructions::create_reward::handler(ctx, args)
    }

    /// Add a reward voucher
    pub fn restaurant_add_reward(ctx: Context<AddRewardVoucher>, args: AddRewardVoucherArgs) -> Result<()> {
        instructions::add_reward::handler(ctx, args)
    }

    /// Remove a reward voucher
    pub fn restaurant_remove_reward(ctx: Context<RemoveRewardVoucher>) -> Result<()> {
        instructions::remove_reward::handler(ctx)
    }

    /// Order Management

    /// Update an order
    pub fn restaurant_update_order(ctx: Context<UpdateCustomerOrder>, args: UpdateOrderArgs) -> Result<()> {
        instructions::update_order::handler(ctx, args)
    }

    /// Customer Functions
    /// These functions handle customer-specific operations

    /// Allow a customer to buy a reward
    pub fn customer_buy_reward(ctx: Context<BuyRewardVoucher>, uri: String) -> Result<()> {
        instructions::buy_reward_voucher::handler(ctx, uri)
    }

    /// Add a new order
    pub fn restaurant_add_order(ctx: Context<AddCustomerOrder>, args: CustomerOrderArgs) -> Result<()> {
        instructions::add_order::handler(ctx, args)
    }

    /// Cancel an existing order
    pub fn restaurant_cancel_order(ctx: Context<CancelCustomerOrder>, args: CancelOrderArgs) -> Result<()> {
        instructions::cancel_order::handler(ctx, args)
    }
}