use anchor_lang::prelude::*;
use crate::state::{AdminProfile, Restaurant, InventoryItem};
use crate::errors::SetupError;

/*
    Remove Inventory Instruction

    Functionality:
    - Allows a restaurant admin to remove an inventory item from their restaurant.
    - Closes the InventoryItem account and returns the rent to the restaurant admin.

    Security checks:
    - Ensures the signer is the restaurant admin.
    - Verifies that the restaurant belongs to the admin.
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RemoveInventoryArgs {
    sku: String,
}

#[derive(Accounts)]
#[instruction(args: RemoveInventoryArgs)]
pub struct RemoveInventory<'info> {
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"inventory", restaurant.key().as_ref(), args.sku.as_ref()],
        bump = item.bump,
    )] 
    pub item: Account<'info, InventoryItem>,

    #[account(mut)]
    pub restaurant_admin: Signer<'info>,

    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ SetupError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveInventory<'info> {
    pub fn remove_inventory(&mut self) -> Result<()> {
        // The actual removal is handled by the `close = restaurant_admin` constraint
        // No additional logic is needed here
        Ok(())
    }
}

pub fn handler(ctx: Context<RemoveInventory>, _args: RemoveInventoryArgs) -> Result<()> {
    ctx.accounts.remove_inventory()
}