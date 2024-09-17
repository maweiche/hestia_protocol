use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Manager, Restaurant, Employee, EmployeeType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddEmployeeArgs {
    wallet: Pubkey,
    restaurant: Pubkey,
    employee_type: u8,
    username: String,
    bump: u8,
}

#[derive(Accounts)]
#[instruction(args: AddEmployeeArgs)]
pub struct AddEmployee<'info> {
    #[account(
        init,
        payer = restaurant_admin,
        space = Employee::INIT_SPACE + args.username.len(),
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

impl<'info> AddEmployee<'info> {
    pub fn add_employee(&mut self, employee_type: EmployeeType, args: AddEmployeeArgs, bump: u8) -> Result<()> {

        self.employee.set_inner(
            Employee {
                wallet: args.wallet,
                restaurant: args.restaurant,
                employee_type,
                username: args.username,
                initialized: true,
                bump
            }
        );

       Ok(())
    }
}

pub fn handler(ctx: Context<AddEmployee>, args: AddEmployeeArgs) -> Result<()> {
    let bump = ctx.bumps.employee;

    let object_type = match args.employee_type{
        0 => EmployeeType::TeamMember,
        1 => EmployeeType::TeamLeader,
        2 => EmployeeType::Manager,
        3 => EmployeeType::Director,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    ctx.accounts.add_employee(object_type, args, bump)?;

    Ok(())
}