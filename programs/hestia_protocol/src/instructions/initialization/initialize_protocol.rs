use anchor_lang::prelude::*;
use crate::state::{Protocol, Manager};
use crate::errors::SetupError;
use crate::constants::admin_wallet as ADMIN;
use mpl_core::types::{OracleValidation, ExternalValidationResult};

/*
    Initialize Protocol Instruction

    Security checks:
    - Verify that the account interacting with this instruction is the multisig account
      of the team with the highest security clearance for the entire protocol.

    Functionality:
    - Initializes the Protocol account with new settings.
    - Initializes the Manager account.

    Note: The ADMIN constant should be defined elsewhere in the codebase.
*/

#[derive(Accounts)]
pub struct ProtocolInit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        init,
        payer = owner,
        space = Protocol::INIT_SPACE,
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    
    #[account(
        init,
        payer = owner,
        space = Manager::INIT_SPACE,
        seeds = [b"manager"],
        bump,
    )]
    pub manager: Account<'info, Manager>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> ProtocolInit<'info> {
    /// Initializes the Protocol account with default validation settings
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

    /// Initializes the Manager account
    fn initialize_manager(&mut self, bump: u8) -> Result<()> {
        self.manager.set_inner(Manager { bump });
        Ok(())
    }
}

pub fn handler(ctx: Context<ProtocolInit>) -> Result<()> {
    // Ensure the initializer is the admin of the protocol
    require!(ctx.accounts.owner.key() == ADMIN::id(), SetupError::Unauthorized);

    // Initialize the new protocol
    ctx.accounts.initialize_protocol(ctx.bumps.protocol)?;

    // Initialize the new manager
    ctx.accounts.initialize_manager(ctx.bumps.manager)?;

    Ok(())
}