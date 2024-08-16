use anchor_lang::prelude::*;
use anchor_spl::associated_token::Create;
use mpl_core::accounts::BaseCollectionV1;
use crate::{state::{AdminProfile, MenuCategoryType, Menu, MenuItem, Restaurant, Employee, EmployeeType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ToggleMenuItemArgs {
    sku: u64,
    active: bool,
}

#[derive(Accounts)]
#[instruction(args: ToggleMenuItemArgs)]
pub struct ToggleMenuItem<'info> {
    #[account(
        mut,
        seeds = [b"item", restaurant.key().as_ref(), args.sku.to_le_bytes().as_ref()],
        bump
    )] 
    pub item: Account<'info, MenuItem>,
    #[account(
        mut,
        seeds = [b"menu", restaurant.key().as_ref()],
        bump,
    )]
    pub menu: Account<'info, Menu>,
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

impl<'info> ToggleMenuItem<'info> {
    pub fn toggle(&mut self, args: ToggleMenuItemArgs) -> Result<()> {

        require!(args.sku == self.item.sku, SetupError::InvalidObjectType);
        
        self.item.active = !self.item.active;

       Ok(())
    }
}

pub fn handler(ctx: Context<ToggleMenuItem>, args: ToggleMenuItemArgs) -> Result<()> {
    ctx.accounts.toggle(args)?;

    Ok(())
}