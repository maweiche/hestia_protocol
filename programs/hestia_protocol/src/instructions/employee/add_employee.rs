use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Restaurant, Employee, EmployeeType}, errors::SetupError};

/*
    Add Employee Instruction

    Functionality:
    - Allows a restaurant admin to add a new employee to their restaurant.
    - Creates a new Employee account with the provided details.

    Security checks:
    - Ensures the signer is the restaurant admin.
    - Verifies that the restaurant belongs to the admin.
*/

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
        constraint = restaurant.owner == *restaurant_admin.key @ SetupError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddEmployee<'info> {
    pub fn add_employee(&mut self, employee_type: EmployeeType, args: AddEmployeeArgs, bump: u8) -> Result<()> {
        self.employee.set_inner(Employee {
            wallet: args.wallet,
            restaurant: args.restaurant,
            employee_type,
            username: args.username,
            initialized: true,
            bump
        });

        Ok(())
    }
}

pub fn handler(ctx: Context<AddEmployee>, args: AddEmployeeArgs) -> Result<()> {
    let employee_type = EmployeeType::from_u8(args.employee_type)
        .ok_or(SetupError::InvalidObjectType)?;

    ctx.accounts.add_employee(employee_type, args, ctx.bumps.employee)
}

// Add this implementation to the EmployeeType enum
impl EmployeeType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::TeamMember),
            1 => Some(Self::TeamLeader),
            2 => Some(Self::Manager),
            3 => Some(Self::Director),
            _ => None,
        }
    }
}