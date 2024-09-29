use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Menu, MenuItem, Restaurant}, errors::MenuError};

/*
    Toggle Menu Item Instruction

    Functionality:
    - Toggles the active status of a menu item (enables or disables it)

    Security checks:
    - Ensures the signer is the restaurant admin
    - Verifies that the restaurant belongs to the admin
    - Checks that the provided SKU matches the menu item's SKU
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ToggleMenuItemArgs {
    sku: String,
}

#[derive(Accounts)]
#[instruction(args: ToggleMenuItemArgs)]
pub struct ToggleMenuItem<'info> {
    #[account(
        mut,
        seeds = [b"item", restaurant.key().as_ref(), args.sku.as_bytes()],
        bump = item.bump,
    )] 
    pub item: Account<'info, MenuItem>,

    #[account(
        mut,
        seeds = [b"menu", restaurant.key().as_ref()],
        bump = menu.bump,
    )]
    pub menu: Account<'info, Menu>,

    #[account(mut)]
    pub restaurant_admin: Signer<'info>,

    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ MenuError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    pub system_program: Program<'info, System>,
}

impl<'info> ToggleMenuItem<'info> {
    pub fn toggle(&mut self, args: ToggleMenuItemArgs) -> Result<()> {
        // Ensure the provided SKU matches the menu item's SKU
        require!(args.sku == self.item.sku, MenuError::InvalidSku);
        
        // Toggle the active status
        self.item.active = !self.item.active;

        // Emit an event for the status change
        emit!(MenuItemToggled {
            sku: self.item.sku.clone(),
            new_status: self.item.active,
            restaurant: self.restaurant.key(),
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<ToggleMenuItem>, args: ToggleMenuItemArgs) -> Result<()> {
    ctx.accounts.toggle(args)
}

// Event emitted when a menu item's status is toggled
#[event]
pub struct MenuItemToggled {
    pub sku: String,
    pub new_status: bool,
    pub restaurant: Pubkey,
}