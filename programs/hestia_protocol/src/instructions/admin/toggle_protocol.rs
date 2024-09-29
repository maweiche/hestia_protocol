use anchor_lang::prelude::*;
use crate::state::Protocol;
use mpl_core::types::{OracleValidation, ExternalValidationResult};

#[derive(Accounts)]
pub struct ProtocolToggle<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"protocol"],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

/*
    
    Change Protocol Validation Ix:

    Some security check:
    - Check if the account that is interacting with this instruction is the mutlisig account 
    of the team that is the highest security clearance for the enitre protocol.

    What these Instructions do:
    - Change the Protocol account with the new settings.

*/

impl<'info> ProtocolToggle<'info> {
    pub fn toggle(&mut self) -> Result<()> { 
        
        if (self.protocol.validation == OracleValidation::V1 { 
            create: ExternalValidationResult::Approved, 
            transfer: ExternalValidationResult::Approved, 
            burn: ExternalValidationResult::Approved, 
            update: ExternalValidationResult::Approved 
        }) {
            self.protocol.validation = OracleValidation::V1 { 
                create: ExternalValidationResult::Rejected, 
                transfer: ExternalValidationResult::Rejected, 
                burn: ExternalValidationResult::Rejected, 
                update: ExternalValidationResult::Rejected 
            };
        } else {
            self.protocol.validation = OracleValidation::V1 { 
                create: ExternalValidationResult::Approved, 
                transfer: ExternalValidationResult::Approved, 
                burn: ExternalValidationResult::Approved, 
                update: ExternalValidationResult::Approved 
            };
        }  

        Ok(())
    }
}

pub fn handler(ctx: Context<ProtocolToggle>) -> Result<()> {
    // Make sure it's the admin of the protocol that is initializing the new admin and that the new admin is not the admin of the protocol
    // require!(ctx.accounts.owner.key() == ADMIN::id(), SetupError::Unauthorized);

    ctx.accounts.toggle()?;

    Ok(())
}
