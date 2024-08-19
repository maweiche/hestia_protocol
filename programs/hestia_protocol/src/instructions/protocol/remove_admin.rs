use anchor_lang::prelude::*;
use crate::{
    state::{
        AdminProfile,
        Protocol,
    },
    constant,
    // errors::{SetupError, ProtocolError},
};

#[derive(Accounts)]
pub struct AdminRemove<'info> {
    /// CHECK: This is the admin being removed, it's ok because the signer will be required to be the overall authority on program
    #[account(mut)]
    pub admin: AccountInfo<'info>,
    #[account(
        mut,
        close = primary_admin, // this is where the account rent funds will be sent to after the admin is removed
        seeds = [b"admin", admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    pub primary_admin: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

/*
        
    Create a new Admin Ix:

    Some security check:
    - Check if the account that is initializing the admin is the admin of the entire protocol.
    - Make sure the admin profile we're creating is not for the admin of the entire protocol, that might be a security issues.
    - Save the Time of initialization to render it useless for the first 16h of initialization.

    What the Instruction does:
    - Initialize the new admin account with the username (so we can monitor who are the admin
    account atm in an easy way) and the publickey of the new admin.

*/


impl<'info> AdminRemove<'info> {
    pub fn remove_admin(
        &mut self
    ) -> Result<()> {

        /*
        
            Remove Admin Ix:

            Some security check:
            - Check if the account signing is the primary admin from the multisig wallet.

            What the Instruction does:
            - Closes the Admin_State account which is necessary for Admin rights, this is intended to only be used when the admin is compromised.
            - Returns any account rent of the Admin_State account to the multisig wallet.   

        */
        
        
        Ok(())
    }
}


pub fn handler(ctx: Context<AdminRemove>) -> Result<()> {
    // Make sure it's the admin of the protocol that is initializing the new admin and that the new admin is not the admin of the protocol
    // require!(ctx.accounts.owner.key() == ADMIN::id() && ctx.accounts.owner.key() != ctx.accounts.new_admin.key(), SetupError::Unauthorized);

    // Generate the bumps
    // let bumps = ctx.bumps;

    // Initialize the new admin_profile
    ctx.accounts.remove_admin()?;

    Ok(())
}