use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Restaurant, RestaurantType}, errors::SetupError};

/*
    Initialize Restaurant Instruction

    Functionality:
    - Creates a new Restaurant account
    - Creates an AdminProfile for the restaurant owner
    - Initializes both accounts with provided data

    Security considerations:
    - Ensures the restaurant_admin is the signer
    - Validates the restaurant type
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateRestaurantArgs {
    id: u64,
    restaurant_type: u8,
    name: String,
    symbol: String,
    currency: Pubkey,
    url: String,
    bump: u8,
}

#[derive(Accounts)]
#[instruction(args: CreateRestaurantArgs)]
pub struct CreateRestaurant<'info> {
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,
    
    #[account(
        init,
        payer = restaurant_admin,
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        space = AdminProfile::INIT_SPACE,
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    
    #[account(
        init,
        payer = restaurant_admin,
        space = Restaurant::INIT_SPACE,
        seeds = [b"restaurant", restaurant_admin.key().as_ref()],
        bump,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    
    pub system_program: Program<'info, System>,
}

impl<'info> CreateRestaurant<'info> {
    /// Initializes the Restaurant account with provided data
    pub fn create_restaurant(&mut self, restaurant_type: RestaurantType, args: CreateRestaurantArgs, bump: u8) -> Result<()> {
        self.restaurant.set_inner(Restaurant {
            restaurant_type,
            owner: *self.restaurant_admin.key,
            name: args.name,
            symbol: args.symbol,
            currency: args.currency,
            url: args.url,
            customer_count: 0,
            bump
        });
        Ok(())
    }

    /// Initializes the AdminProfile account for the restaurant owner
    pub fn create_owner(&mut self, bump: u8) -> Result<()> {
        self.admin_profile.set_inner(AdminProfile {
            username: "owner".to_string(),
            creation_time: Clock::get()?.unix_timestamp - 20 * 60 * 60,
            bump
        });
        Ok(())
    }
}

pub fn handler(ctx: Context<CreateRestaurant>, args: CreateRestaurantArgs) -> Result<()> {
    // Convert restaurant_type from u8 to RestaurantType enum
    let restaurant_type = match args.restaurant_type {
        0 => RestaurantType::Foodtruck,
        1 => RestaurantType::Cafe,
        2 => RestaurantType::Restaurant,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    // Initialize the Restaurant account
    ctx.accounts.create_restaurant(restaurant_type, args, ctx.bumps.restaurant)?;

    // Initialize the AdminProfile account
    ctx.accounts.create_owner(ctx.bumps.admin_profile)?;

    Ok(())
}