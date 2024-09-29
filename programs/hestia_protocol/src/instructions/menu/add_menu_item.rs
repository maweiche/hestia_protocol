use anchor_lang::prelude::*;
use crate::{
    state::{AdminProfile, MenuCategoryType, Menu, MenuItem, Restaurant, IngredientList},
    errors::SetupError
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MenuItemArgs {
    sku: String,
    category: u8,
    name: String,
    price: u64,
    description: String,
    ingredients: Vec<(Pubkey, u64)>, // (InventoryItem pubkey, quantity)
    active: bool,
}

#[derive(Accounts)]
#[instruction(args: MenuItemArgs)]
pub struct ManageMenuItem<'info> {
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = MenuItem::INIT_SPACE + args.sku.len() + args.name.len() + args.description.len(),
        seeds = [b"menu_item", args.sku.as_bytes()],
        bump,
    )] 
    pub menu_item: Account<'info, MenuItem>,

    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = IngredientList::INIT_SPACE + (args.ingredients.len() * 32),
        seeds = [b"ingredient_list", menu_item.key().as_ref()],
        bump,
    )]
    pub ingredient_list: Account<'info, IngredientList>,

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
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ SetupError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    pub system_program: Program<'info, System>,
}

impl<'info> ManageMenuItem<'info> {
    pub fn add_menu_item(&mut self, category: MenuCategoryType, args: MenuItemArgs, menu_item_bump: u8, ingredient_list_bump: u8 ) -> Result<()> {
        if !self.menu.initialized {
            self.menu.set_inner(Menu {
                bump: menu_item_bump,
                initialized: true,
            });
        }

        self.menu_item.set_inner(MenuItem {
            sku: args.sku.clone(),
            category,
            name: args.name,
            price: args.price,
            description: args.description,
            active: args.active,
            bump: menu_item_bump,
        });

        self.ingredient_list.set_inner(IngredientList {
            menu_item: self.menu_item.key(),
            ingredients: args.ingredients,
            bump: ingredient_list_bump
        });

        Ok(())
    }

    pub fn update_menu_item(&mut self, category: MenuCategoryType, args: MenuItemArgs) -> Result<()> {
        self.menu_item.category = category;
        self.menu_item.name = args.name;
        self.menu_item.price = args.price;
        self.menu_item.description = args.description;
        self.menu_item.active = args.active;

        self.ingredient_list.ingredients = args.ingredients;

        Ok(())
    }
}

pub fn add_menu_item_handler(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    let bumps = ctx.bumps;
    let category = MenuCategoryType::_from_u8(args.category)
        .ok_or(SetupError::InvalidObjectType)?;

    ctx.accounts.add_menu_item(category, args, bumps.menu_item, bumps.ingredient_list)
}

pub fn update_menu_item_handler(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    let category = MenuCategoryType::_from_u8(args.category)
        .ok_or(SetupError::InvalidObjectType)?;

    ctx.accounts.update_menu_item(category, args)
}

impl MenuCategoryType {
    fn _from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Combo),
            1 => Some(Self::Side),
            2 => Some(Self::Entree),
            3 => Some(Self::Dessert),
            4 => Some(Self::Beverage),
            5 => Some(Self::Alcohol),
            6 => Some(Self::Other),
            _ => None,
        }
    }
}