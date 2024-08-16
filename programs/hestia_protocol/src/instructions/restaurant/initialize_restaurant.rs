use anchor_lang::prelude::*;
// import clock sysvar
use anchor_spl::associated_token::Create;
use mpl_core::accounts::BaseCollectionV1;
use crate::{state::{AdminProfile, Manager, Restaurant, RestaurantType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateRestaurantArgs {
    id: u64,
    restaurant_type: u8,
    name: String,
    symbol: String,
    currency: Pubkey,
    url: String,
    customer_count: u64,
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
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
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
    pub fn create_restaurant(&mut self, restaurant_type: RestaurantType, args: CreateRestaurantArgs, bump: u8) -> Result<()> {

        self.restaurant.set_inner(
            Restaurant {
                restaurant_type,
                owner: *self.restaurant_admin.key,
                name: args.name,
                symbol: args.symbol,
                currency: args.currency,
                url: args.url,
                customer_count: args.customer_count,
                bump
            }   
        );

       Ok(())
    }

    pub fn create_owner(&mut self, bump: u8) -> Result<()> {

        self.admin_profile.set_inner(
            AdminProfile {
                username: "owner".to_string(),
                creation_time: Clock::get()?.unix_timestamp - 20 * 60 * 60,
                bump
            }
        );

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateRestaurant>, args: CreateRestaurantArgs) -> Result<()> {
    let restaurant_bump = ctx.bumps.restaurant;
    let owner_bump = ctx.bumps.admin_profile;

    let object_type = match args.restaurant_type{
        0 => RestaurantType::Foodtruck,
        1 => RestaurantType::Cafe,
        2 => RestaurantType::Restaurant,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    ctx.accounts.create_restaurant(object_type, args, restaurant_bump)?;
    ctx.accounts.create_owner(owner_bump)?;

    Ok(())
}