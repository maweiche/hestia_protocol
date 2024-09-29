use anchor_lang::prelude::*;
use crate::state::Protocol;
use crate::errors::SetupError;
use mpl_core::types::{OracleValidation, ExternalValidationResult};
use crate::constants::admin_wallet as ADMIN;

/*
    Toggle Protocol Validation Instruction

    Security check:
    - Verify that the account interacting with this instruction is the multisig account 
      of the team with the highest security clearance for the entire protocol.

    Functionality:
    - Toggles the Protocol account validation settings between Approved and Rejected states.
*/

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

impl<'info> ProtocolToggle<'info> {
    pub fn toggle(&mut self) -> Result<()> {
        let new_validation = match &self.protocol.validation {
            OracleValidation::V1 { create, transfer, burn, update } 
            if *create == ExternalValidationResult::Approved 
                && *transfer == ExternalValidationResult::Approved 
                && *burn == ExternalValidationResult::Approved 
                && *update == ExternalValidationResult::Approved => {
                OracleValidation::V1 {
                    create: ExternalValidationResult::Rejected,
                    transfer: ExternalValidationResult::Rejected,
                    burn: ExternalValidationResult::Rejected,
                    update: ExternalValidationResult::Rejected,
                }
            },
            _ => OracleValidation::V1 {
                create: ExternalValidationResult::Approved,
                transfer: ExternalValidationResult::Approved,
                burn: ExternalValidationResult::Approved,
                update: ExternalValidationResult::Approved,
            },
        };
    
        self.protocol.validation = new_validation;
        Ok(())
    }
}

pub fn handler(ctx: Context<ProtocolToggle>) -> Result<()> {
    // Verify that the signer is the admin of the protocol
    require!(ctx.accounts.owner.key() == ADMIN::id(), SetupError::Unauthorized);

    ctx.accounts.toggle()?;
    Ok(())
}