use anchor_lang::prelude::*;
use crate::{state::{CustomerOrder, Restaurant, StatusType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateOrderArgs {
    order_id: u64,
    status: u8,
    bump: u8
}

#[derive(Accounts)]
#[instruction(args: UpdateOrderArgs)]
pub struct UpdateCustomerOrder<'info> {
    #[account(
        mut,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref()],
        bump,
    )] 
    pub order: Account<'info, CustomerOrder>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateCustomerOrder<'info> {
    pub fn update_order(&mut self, status: StatusType) -> Result<()> {
        // TODO() - ADD SECURITY CHECK TO MAKE SURE SIGNER IS RESTAURANT EMPLOYEE
        self.order.status = status;
        self.order.updated_at = Some(Clock::get()?.unix_timestamp - 20 * 60 * 60);

       Ok(())
    }
}

pub fn handler(ctx: Context<UpdateCustomerOrder>, _args: UpdateOrderArgs) -> Result<()> {
    let args = _args;
    let status_type = match args.status{
        0 => StatusType::Pending,
        1 => StatusType::Completed,
        2 => StatusType::Finalized,
        3 => StatusType::Cancelled,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };


    ctx.accounts.update_order(status_type)?;

    Ok(())
}