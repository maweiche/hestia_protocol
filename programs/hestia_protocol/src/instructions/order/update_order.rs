use anchor_lang::prelude::*;
use crate::{
    state::{CustomerOrder, Restaurant, StatusType, Employee, EmployeeType},
    errors::OrderError
};

/*
    Update Customer Order Instruction

    Functionality:
    - Allows a restaurant employee to update the status of an order
    - Updates the order's status and timestamp

    Security checks:
    - Ensures the signer is a valid restaurant employee
    - Verifies that the order belongs to the correct restaurant
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateOrderArgs {
    order_id: u64,
    status: u8,
}

#[derive(Accounts)]
#[instruction(args: UpdateOrderArgs)]
pub struct UpdateCustomerOrder<'info> {
    #[account(
        mut,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref()],
        bump = order.bump,
    )] 
    pub order: Account<'info, CustomerOrder>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,

    #[account(
        seeds = [b"employee", restaurant.key().as_ref(), signer.key().as_ref()],
        bump = employee.bump,
        constraint = employee.employee_type != EmployeeType::TeamMember @ OrderError::Unauthorized,
    )]
    pub employee: Account<'info, Employee>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateCustomerOrder<'info> {
    pub fn update_order(&mut self, status: StatusType) -> Result<()> {
        self.order.status = status;
        self.order.updated_at = Some(Clock::get()?.unix_timestamp);

        Ok(())
    }
}

pub fn handler(ctx: Context<UpdateCustomerOrder>, args: UpdateOrderArgs) -> Result<()> {
    let status_type = StatusType::from_u8(args.status)
        .ok_or(OrderError::InvalidStatusType)?;

    ctx.accounts.update_order(status_type)
}

impl StatusType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Pending),
            1 => Some(Self::Completed),
            2 => Some(Self::Finalized),
            3 => Some(Self::Cancelled),
            _ => None,
        }
    }
}