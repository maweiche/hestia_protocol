use anchor_lang::{
    prelude::*, 
    solana_program::{
        program_memory::sol_memcpy,
        sysvar::instructions::{
            self,
            load_current_index_checked,
            load_instruction_at_checked
        }
    }, Bump
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, TokenAccount, Token, transfer, Transfer}
};
use std::str::FromStr;
use anchor_spl::associated_token::Create;
use mpl_core::accounts::BaseCollectionV1;
use crate::{state::{Customer, CustomerOrder, Restaurant, StatusType, EmployeeType, InventoryCategoryType, InventoryItem}, errors::{SetupError, BuyingError},
constants::{
    // protocol_currency, 
    signing_authority, 
    ED25519_PROGRAM_ID
},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DeleteOrderArgs {
    order_id: u64,
    customer: Pubkey,
    bump: u8
}

#[derive(Accounts)]
#[instruction(args: DeleteOrderArgs)]
pub struct DeleteCustomerOrder<'info> {
    #[account(
        mut,
        close = signer,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub order: Account<'info, CustomerOrder>,
    #[account(
        mut,
        seeds = [b"customer", restaurant.key().as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub customer: Account<'info, Customer>,
    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DeleteCustomerOrder<'info> {
    pub fn delete_order(&mut self, args: DeleteOrderArgs, bump: u8) -> Result<()> {

        // Close the order account
        // TOD0

        self.order.close(
            self.signer.to_account_info()
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<DeleteCustomerOrder>, args: DeleteOrderArgs) -> Result<()> {
    let bump = ctx.bumps.order;
    

    ctx.accounts.delete_order(args, bump)?;

    Ok(())
}