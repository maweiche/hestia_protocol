use anchor_lang::prelude::*;
use crate::state::{AdminProfile, Restaurant, InventoryItem};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RemoveInventoryArgs {
    sku: String,
}

#[derive(Accounts)]
pub struct RemoveInventory<'info> {
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"inventory", restaurant.key().as_ref(), item.sku.as_ref()],
        bump,
    )] 
    pub item: Account<'info, InventoryItem>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        constraint = restaurant.owner == *restaurant_admin.key,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveInventory<'info> {
    pub fn remove_inventory(&mut self) -> Result<()> {
       Ok(())
    }
}

pub fn handler(ctx: Context<RemoveInventory>) -> Result<()> {
    ctx.accounts.remove_inventory()?;
    Ok(())
}