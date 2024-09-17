use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Manager, Restaurant, Employee, EmployeeType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DeleteEmployeeArgs {
    wallet: Pubkey,
    restaurant: Pubkey,
    bump: u8,
}

#[derive(Accounts)]
#[instruction(args: DeleteEmployeeArgs)]
pub struct DeleteEmployee<'info> {
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"employee", restaurant.key().as_ref(), args.wallet.as_ref()],
        bump,
    )] 
    pub employee: Account<'info, Employee>,
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        constraint = restaurant.owner == *restaurant_admin.key,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    pub system_program: Program<'info, System>,
}

impl<'info> DeleteEmployee<'info> {
    pub fn delete_employee(&mut self, args: DeleteEmployeeArgs, bump: u8) -> Result<()> {

        self.employee.close(
            self.restaurant_admin.to_account_info()
        )?;

       Ok(())
    }
}

pub fn handler(ctx: Context<DeleteEmployee>, args: DeleteEmployeeArgs) -> Result<()> {
    let bump = ctx.bumps.employee;

    ctx.accounts.delete_employee(args, bump)?;

    Ok(())
}