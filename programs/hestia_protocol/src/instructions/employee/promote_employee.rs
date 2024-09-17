use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Manager, Restaurant, Employee, EmployeeType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct PromoteEmployeeArgs {
    wallet: Pubkey,
    restaurant: Pubkey,
    employee_type: u8,
    bump: u8,
}

#[derive(Accounts)]
#[instruction(args: PromoteEmployeeArgs)]
pub struct PromoteEmployee<'info> {
    #[account(
        mut,
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

impl<'info> PromoteEmployee<'info> {
    pub fn promote_employee(&mut self, employee_type: EmployeeType, args: PromoteEmployeeArgs, bump: u8) -> Result<()> {

        self.employee.employee_type = employee_type;

       Ok(())
    }
}

pub fn handler(ctx: Context<PromoteEmployee>, args: PromoteEmployeeArgs) -> Result<()> {
    let bump = ctx.bumps.employee;

    let object_type = match args.employee_type{
        0 => EmployeeType::TeamMember,
        1 => EmployeeType::TeamLeader,
        2 => EmployeeType::Manager,
        3 => EmployeeType::Director,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    ctx.accounts.promote_employee(object_type, args, bump)?;

    Ok(())
}