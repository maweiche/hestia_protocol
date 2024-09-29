use anchor_lang::prelude::*;
use crate::{
    state::{AdminProfile, MenuCategoryType, Menu, MenuItem, Restaurant, IngredientList},
    errors::SetupError
};

/*
    Manage Menu Item Instruction

    Functionality:
    - Adds a new menu item or updates an existing one
    - Creates or updates the associated IngredientList
    - Initializes the restaurant's Menu if it doesn't exist

    Security checks:
    - Ensures the signer is the restaurant admin
    - Verifies that the restaurant belongs to the admin
    - Uses PDAs to ensure proper ownership and access control

    Note: This instruction handles both adding new menu items and updating existing ones.
*/

/// Arguments for adding or updating a menu item
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

/// Accounts required for managing (adding or updating) a menu item
#[derive(Accounts)]
#[instruction(args: MenuItemArgs)]
pub struct ManageMenuItem<'info> {
    /// The menu item account, initialized if it doesn't exist
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = MenuItem::INIT_SPACE + args.sku.len() + args.name.len() + args.description.len(),
        seeds = [b"menu_item", args.sku.as_bytes()],
        bump,
    )] 
    pub menu_item: Account<'info, MenuItem>,

    /// The ingredient list account for the menu item, initialized if it doesn't exist
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = IngredientList::INIT_SPACE + (args.ingredients.len() * 32),
        seeds = [b"ingredient_list", menu_item.key().as_ref()],
        bump,
    )]
    pub ingredient_list: Account<'info, IngredientList>,

    /// The restaurant's menu account, initialized if it doesn't exist
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = Menu::INIT_SPACE,
        seeds = [b"menu", restaurant.key().as_ref()],
        bump,
    )]
    pub menu: Account<'info, Menu>,

    /// The restaurant admin who is adding or updating the menu item
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,

    /// The admin profile of the restaurant admin
    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    /// The restaurant account, ensuring the admin is authorized
    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ SetupError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    /// The system program
    pub system_program: Program<'info, System>,
}

impl<'info> ManageMenuItem<'info> {
    /// Adds a new menu item
    pub fn add_menu_item(&mut self, category: MenuCategoryType, args: MenuItemArgs, menu_item_bump: u8, ingredient_list_bump: u8 ) -> Result<()> {
        // Initialize the menu if it hasn't been initialized yet
        if !self.menu.initialized {
            self.menu.set_inner(Menu {
                bump: menu_item_bump,
                initialized: true,
            });
        }

        // Set the menu item details
        self.menu_item.set_inner(MenuItem {
            sku: args.sku.clone(),
            category,
            name: args.name,
            price: args.price,
            description: args.description,
            active: args.active,
            bump: menu_item_bump,
        });

        // Set the ingredient list for the menu item
        self.ingredient_list.set_inner(IngredientList {
            menu_item: self.menu_item.key(),
            ingredients: args.ingredients,
            bump: ingredient_list_bump
        });

        Ok(())
    }

    /// Updates an existing menu item
    pub fn update_menu_item(&mut self, category: MenuCategoryType, args: MenuItemArgs) -> Result<()> {
        // Update the menu item details
        self.menu_item.category = category;
        self.menu_item.name = args.name;
        self.menu_item.price = args.price;
        self.menu_item.description = args.description;
        self.menu_item.active = args.active;

        // Update the ingredient list
        self.ingredient_list.ingredients = args.ingredients;

        Ok(())
    }
}

/// Handler for adding a new menu item
pub fn add_menu_item_handler(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    let bumps = ctx.bumps;
    let category = MenuCategoryType::_from_u8(args.category)
        .ok_or(SetupError::InvalidObjectType)?;

    ctx.accounts.add_menu_item(category, args, bumps.menu_item, bumps.ingredient_list)
}

/// Handler for updating an existing menu item
pub fn update_menu_item_handler(ctx: Context<ManageMenuItem>, args: MenuItemArgs) -> Result<()> {
    let category = MenuCategoryType::_from_u8(args.category)
        .ok_or(SetupError::InvalidObjectType)?;

    ctx.accounts.update_menu_item(category, args)
}

impl MenuCategoryType {
    /// Converts a u8 value to a MenuCategoryType
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