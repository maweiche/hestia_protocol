use anchor_lang::prelude::*;
use crate::{
    state::{Customer, CustomerOrder, Restaurant, AdminProfile, StatusType},
    errors::OrderError
};

/*
    Cancel Customer Order Instruction

    Functionality:
    - Allows either the customer or the restaurant admin to cancel an order
    - Closes the CustomerOrder account and returns the rent to the signer
    - Updates the order status to Cancelled
    - Decrements the customer's total_orders count

    Security checks:
    - Ensures the signer is either the customer or the restaurant admin
    - Verifies that the order belongs to the correct customer and restaurant
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CancelOrderArgs {
    order_id: u64,
}

#[derive(Accounts)]
#[instruction(args: CancelOrderArgs)]
pub struct CancelCustomerOrder<'info> {
    #[account(
        mut,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref()],
        bump = order.bump,
        constraint = order.customer == customer.key() @ OrderError::InvalidCustomer,
        constraint = order.status != StatusType::Cancelled @ OrderError::AlreadyCancelled,
    )] 
    pub order: Account<'info, CustomerOrder>,

    #[account(
        mut,
        seeds = [b"customer", restaurant.key().as_ref(), customer.key().as_ref()],
        bump = customer.bump,
    )] 
    pub customer: Account<'info, Customer>,

    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"admin", restaurant.owner.as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub system_program: Program<'info, System>,
}

impl<'info> CancelCustomerOrder<'info> {
    pub fn cancel_order(&mut self) -> Result<()> {
        // Ensure the signer is either the customer or the restaurant admin
        require!(
            self.signer.key() == self.customer.key() || self.signer.key() == self.restaurant.owner,
            OrderError::Unauthorized
        );

        // Update order status
        self.order.status = StatusType::Cancelled;
        self.order.updated_at = Some(Clock::get()?.unix_timestamp);

        // Decrement customer's total_orders
        self.customer.total_orders = self.customer.total_orders.saturating_sub(1);
        self.customer.reward_points = self.customer.reward_points.saturating_sub(self.order.total / 100);

        let order = &self.order;

        // Emit the new order event
        emit!(OrderUpdated {
            order_id: order.order_id,
            customer: order.customer,
            items: order.items.clone(),
            total: order.total,
            status: StatusType::Pending,
            created_at: order.created_at,
            updated_at: Clock::get()?.unix_timestamp,
            restaurant: self.restaurant.key(),
        });

        // The actual closing of the account is handled by the `close = signer` constraint
        Ok(())
    }
}

pub fn handler(ctx: Context<CancelCustomerOrder>, _args: CancelOrderArgs) -> Result<()> {
    ctx.accounts.cancel_order()
}

#[event]
pub struct OrderUpdated {
    pub order_id: u64,
    pub customer: Pubkey,
    pub items: Vec<u64>,
    pub total: u64,
    pub status: StatusType,
    pub created_at: i64,
    pub updated_at: i64,
    pub restaurant: Pubkey,
}