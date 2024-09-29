use anchor_lang::prelude::*;
use crate::state::{AdminProfile, Restaurant, Employee};
use crate::errors::SetupError;

/*
    Remove Employee Instruction

    Functionality:
    - Allows a restaurant admin to remove an employee from their restaurant.
    - Closes the Employee account and returns the rent to the restaurant admin.

    Security checks:
    - Ensures the signer is the restaurant admin.
    - Verifies that the restaurant belongs to the admin.
    - Checks that the employee belongs to the specified restaurant.
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct RemoveEmployeeArgs {
    wallet: Pubkey,
    restaurant: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: RemoveEmployeeArgs)]
pub struct RemoveEmployee<'info> {
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"employee", restaurant.key().as_ref(), args.wallet.as_ref()],
        bump = employee.bump,
        constraint = employee.restaurant == args.restaurant @ SetupError::EmployeeMismatch,
    )] 
    pub employee: Account<'info, Employee>,

    #[account(mut)]
    pub restaurant_admin: Signer<'info>,

    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ SetupError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveEmployee<'info> {
    pub fn remove_employee(&mut self) -> Result<()> {
        // The actual removal is handled by the `close = restaurant_admin` constraint
        // No additional logic is needed here
        Ok(())
    }
}

pub fn handler(ctx: Context<RemoveEmployee>, _args: RemoveEmployeeArgs) -> Result<()> {
    ctx.accounts.remove_employee()
}