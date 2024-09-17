use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, MenuCategoryType, Menu, MenuItem, Restaurant, Employee, EmployeeType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddMenuItemArgs {
    sku: String,
    category: u8,
    name: String,
    price: u64,
    description: String,
    ingredients: Vec<String>,
    active: bool,
}

#[derive(Accounts)]
#[instruction(args: AddMenuItemArgs)]
pub struct AddMenuItem<'info> {
    #[account(
        init,
        payer = restaurant_admin,
        space = MenuItem::INIT_SPACE + args.sku.len() + args.name.len() + args.description.len() + (args.ingredients.len() * 8),
        seeds = [b"item", args.sku.as_bytes().as_ref()],
        bump,
    )] 
    pub item: Account<'info, MenuItem>,
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = Menu::INIT_SPACE,
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

impl<'info> AddMenuItem<'info> {
    pub fn add_item(&mut self, category: MenuCategoryType, args: AddMenuItemArgs, item_bump: u8, menu_bump: u8) -> Result<()> {

        if self.menu.initialized == false {
            self.menu.set_inner(
                Menu {
                    bump: menu_bump,
                    initialized: true,
                }
            );
        }

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

pub fn handler(ctx: Context<AddMenuItem>, args: AddMenuItemArgs) -> Result<()> {
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

    ctx.accounts.add_item(menu_category_type, arguments, item_bump, menu_bump)?;

    Ok(())
}