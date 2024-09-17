pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("93UGwDhoXvZxYuxHq47aWKFcQTgxTM8V55ytPnET1Qjd");

#[program]
pub mod hestia_protocol {
    use super::*;
    
    /////////////////////////////////////////////////////////////
    ////////////////PROTOCOL FUNCTIONS///////////////////////////
    ////////////////////////////////////////////////////////////
    
    // inits protocol and sets up the global state with an admin
    pub fn protocol_init(ctx: Context<ProtocolInit>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx)
    }

    // unlock/lock protocol -- no more changes can be made to the protocol in case of emergency
    pub fn protocol_toggle(ctx: Context<ProtocolToggle>) -> Result<()> {
        instructions::toggle_protocol::handler(ctx)
    }

    // add protocol admin -- can only be called a current admin, gives perms to add new admins/restaurants
    pub fn protocol_add_admin(ctx: Context<AdminInit>, username: String) -> Result<()> {
        instructions::initialize_admin::handler(ctx, username)
    }

    // remove protocol admin -- can only be called a current admin, removes perms to add new admins/restaurants
    pub fn protocol_remove_admin(ctx: Context<AdminRemove>) -> Result<()> {
        instructions::remove_admin::handler(ctx)
    }

    /////////////////////////////////////////////////////////////
    ////////////////RESTAURANT FUNCTIONS/////////////////////////
    ////////////////////////////////////////////////////////////

    // inits restaurant to the hestia protocol and sets up the global state with an admin
    pub fn restaurant_initialize(ctx: Context<CreateRestaurant>, args: CreateRestaurantArgs) -> Result<()> {
        instructions::initialize_restaurant::handler(ctx, args)
    }

    // // add restaurant employee -- initializes employee and gives perms to execute employee functions based on employeeType enum
    pub fn restaurant_add_employee(ctx: Context<AddEmployee>, args: AddEmployeeArgs) -> Result<()> {
        instructions::add_employee::handler(ctx, args)
    }

    // // remove restaurant employee -- removes perms to execute employee functions
    pub fn restaurant_remove_employee(ctx: Context<DeleteEmployee>, args: DeleteEmployeeArgs) -> Result<()> {
        instructions::remove_employee::handler(ctx, args)
    }

    // // promote restaurant employee -- removes perms to execute employee functions
    pub fn restaurant_promote_employee(ctx: Context<PromoteEmployee>, args: PromoteEmployeeArgs) -> Result<()> {
        instructions::promote_employee::handler(ctx, args)
    }
 
    // // add inventory item -- adds an item to the restaurant's inventory, category based on enum
    pub fn restaurant_add_inventory_item(ctx: Context<AddInventory>, args: AddInventoryArgs) -> Result<()> {
        instructions::add_inventory::handler(ctx, args)
    }

    // // update an inventory item -- updates an item in the restaurant's inventory
    pub fn restaurant_update_inventory_item(ctx: Context<AddInventory>, args: AddInventoryArgs) -> Result<()> {
        instructions::add_inventory::handler(ctx, args)
    }

    // // remove an inventory item -- removes an item from the restaurant's inventory
    // pub fn restaurant_remove_inventory_item(ctx: Context<Initialize>) -> Result<()> {}

    // // add menu item -- adds an item to the restaurant's menu, category based on enum
    pub fn restaurant_add_menu_item(ctx: Context<AddMenuItem>, args: AddMenuItemArgs) -> Result<()> {
        instructions::add_menu_item::handler(ctx, args)
    }

    // // update a menu item -- updates an item in the restaurant's menu
    // pub fn restaurant_update_menu_item(ctx: Context<Initialize>) -> Result<()> {}

    // // toggle a menu item -- removes an item from the restaurant's menu
    pub fn restaurant_toggle_menu_item(ctx: Context<ToggleMenuItem>, args: ToggleMenuItemArgs) -> Result<()> {
        instructions::toggle_menu_item::handler(ctx, args)
    }

    // // add reward -- adds a reward to the restaurant's rewards
    // pub fn restaurant_add_reward(ctx: Context<Initialize>) -> Result<()> {}

    // // remove reward -- removes a reward from the restaurant's rewards
    // pub fn restaurant_remove_reward(ctx: Context<Initialize>) -> Result<()> {}

    // // add customer -- adds a customer to the restaurant's customer list
    // pub fn restaurant_add_customer(ctx: Context<Initialize>) -> Result<()> {}
   
    // // update order -- updates an order in the restaurant's order list
    // pub fn restaurant_update_order(ctx: Context<Initialize>) -> Result<()> {}

    // // close order -- closes an order in the restaurant's order list
    // pub fn restaurant_close_order(ctx: Context<Initialize>) -> Result<()> {}

    // //////////////////////////////////////////////////////////////
    // //////////////////CUSTOMER FUNCTIONS/////////////////////////
    // ////////////////////////////////////////////////////////////

    // // buy reward -- allows a customer to buy a reward
    // pub fn customer_buy_reward(ctx: Context<Initialize>) -> Result<()> {}

    // // add order -- adds an order to the restaurant's order list
    pub fn restaurant_add_order(ctx: Context<AddCustomerOrder>, args: CustomerOrderArgs) -> Result<()> {
        instructions::add_order::handler(ctx, args)
    }

    // // cancel order -- cancels an order in the restaurant's order list
    pub fn restaurant_cancel_order(ctx: Context<DeleteCustomerOrder>, args: DeleteOrderArgs) -> Result<()> {
        instructions::cancel_order::handler(ctx, args)
    }
}
