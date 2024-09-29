use anchor_lang::prelude::*;
use crate::state::{ AdminProfile, Protocol };
use crate::errors::SetupError;
use crate::constants::admin_wallet as ADMIN;

/*
    Remove Admin Instruction

    Security checks:
    - Verify that the account removing the admin is the primary admin (from the multisig wallet).
    - Ensure the admin being removed is not the primary admin of the protocol.

    Functionality:
    - Closes the AdminProfile account of the admin being removed.
    - Returns the account rent of the AdminProfile to the primary admin's account.

    Note: This instruction should only be used when an admin account is compromised.
*/

#[derive(Accounts)]
pub struct AdminRemove<'info> {
    /// The admin being removed
    #[account(mut)]
    pub admin: AccountInfo<'info>,

    #[account(
        mut,
        close = primary_admin,
        seeds = [b"admin", admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(mut)]
    pub primary_admin: Signer<'info>,

    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,

    pub system_program: Program<'info, System>,
}

impl<'info> AdminRemove<'info> {
    /// Removes the admin by closing their AdminProfile account
    pub fn remove_admin(&mut self) -> Result<()> {
        // The actual removal is handled by the `close = primary_admin` constraint
        // in the AdminRemove struct. No additional logic is needed here.
        Ok(())
    }
}

pub fn handler(ctx: Context<AdminRemove>) -> Result<()> {
    // Verify that the signer is the primary admin
    require!(
        ctx.accounts.primary_admin.key() == ADMIN::id(),
        SetupError::Unauthorized
    );

    // Ensure the admin being removed is not the primary admin
    require!(
        ctx.accounts.admin.key() != ADMIN::id(),
        SetupError::CannotRemovePrimaryAdmin
    );

    // The actual removal is handled by the Account constraints
    // We just need to call the remove_admin function to satisfy the instruction
    ctx.accounts.remove_admin()?;

    Ok(())
}