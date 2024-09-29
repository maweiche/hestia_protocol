use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, MenuCategoryType, Menu, MenuItem, Restaurant}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateMenuItemArgs {
    sku: String,
    category: u8,
    name: String,
    price: u64,
    description: String,
    ingredients: Vec<String>,
    active: bool,
}

#[derive(Accounts)]
#[instruction(args: UpdateMenuItemArgs)]
pub struct UpdateMenuItem<'info> {
    #[account(
        mut,
        seeds = [b"item", args.sku.as_bytes().as_ref()],
        bump,
    )] 
    pub item: Account<'info, MenuItem>,
    #[account(
        mut,
        seeds = [b"menu", restaurant.key().as_ref()],
        bump,
    )]
    pub menu: Account<'info, Menu>,
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

impl<'info> UpdateMenuItem<'info> {
    pub fn update_item(&mut self, category: MenuCategoryType, args: UpdateMenuItemArgs, item_bump: u8, menu_bump: u8) -> Result<()> {

        self.item.set_inner(
            MenuItem {
                sku: args.sku,
                category,
                name: args.name,
                price: args.price,
                description: args.description,
                ingredients: args.ingredients.iter().map(|x| x.parse().unwrap()).collect(),
                active: args.active,
                bump: item_bump,
            }
        );

       Ok(())
    }
}

pub fn handler(ctx: Context<UpdateMenuItem>, args: UpdateMenuItemArgs) -> Result<()> {
    let item_bump = ctx.bumps.item;
    let menu_bump = ctx.bumps.menu;
    let arguments = args;
    let menu_category_type = match arguments.category{
        0 => MenuCategoryType::Combo,
        1 => MenuCategoryType::Side,
        2 => MenuCategoryType::Entree,
        3 => MenuCategoryType::Dessert,
        4 => MenuCategoryType::Beverage,
        5 => MenuCategoryType::Alcohol,
        6 => MenuCategoryType::Other,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    ctx.accounts.update_item(menu_category_type, arguments, item_bump, menu_bump)?;

    Ok(())
}