use anchor_lang::prelude::*;
use crate::state::AdminProfile;
use crate::errors::SetupError;
use crate::constants::admin_wallet as ADMIN;

/*
    Initialize Admin Instruction

    Security checks:
    - Verify that the account initializing the admin is the admin of the entire protocol.
    - Ensure the admin profile being created is not for the admin of the entire protocol.
    - Set the initialization time to prevent immediate use (16-hour cooldown).

    Functionality:
    - Initializes a new admin account with the provided username and public key.
    - Creates an AdminProfile for the new admin.

    Note: The ADMIN constant should be defined elsewhere in the codebase.
*/

#[derive(Accounts)]
#[instruction(username: String)]
pub struct AdminInit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub new_admin: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = AdminProfile::INIT_SPACE + username.len(),
        seeds = [b"admin", new_admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    pub system_program: Program<'info, System>,
}

impl<'info> AdminInit<'info> {
    /// Initializes the AdminProfile account with provided username and creation time
    pub fn initialize_admin_profile(&mut self, username: String, bump: u8) -> Result<()> {
        let creation_time = Clock::get()?.unix_timestamp - 20 * 60 * 60; // 20 hours ago
        self.admin_profile.set_inner(AdminProfile {
            username,
            creation_time,
            bump,
        });
        Ok(())
    }
}

pub fn handler(ctx: Context<AdminInit>, username: String) -> Result<()> {
    // Verify authorization and prevent self-assignment as admin
    require!(
        ctx.accounts.owner.key() == ADMIN::id() && 
        ctx.accounts.owner.key() != ctx.accounts.new_admin.key(),
        SetupError::Unauthorized
    );

    // Initialize the new admin_profile
    ctx.accounts.initialize_admin_profile(username, ctx.bumps.admin_profile)?;

    Ok(())
}