use anchor_lang::prelude::*;
use crate::state::{Customer, CustomerOrder, Restaurant};

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
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref()],
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
    pub fn delete_order(&mut self) -> Result<()> {

        Ok(())
    }
}

pub fn handler(ctx: Context<DeleteCustomerOrder>, args: DeleteOrderArgs) -> Result<()> {
    let bump = ctx.bumps.order;
    

    ctx.accounts.delete_order()?;

    Ok(())
}