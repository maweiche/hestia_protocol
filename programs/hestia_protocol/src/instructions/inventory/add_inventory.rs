use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Restaurant, InventoryCategoryType, InventoryItem}, errors::SetupError};

/*
    Add/Update Inventory Instruction

    Functionality:
    - Allows a restaurant admin to add a new inventory item or update an existing one.
    - Creates or updates an InventoryItem account with the provided details.

    Security checks:
    - Ensures the signer is the restaurant admin.
    - Verifies that the restaurant belongs to the admin.
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InventoryArgs {
    sku: String,
    category: u8,
    name: String,
    price: u64,
    stock: u64,
    initialized: bool,
}

#[derive(Accounts)]
#[instruction(args: InventoryArgs)]
pub struct ManageInventory<'info> {
    #[account(
        init_if_needed,
        payer = restaurant_admin,
        space = InventoryItem::INIT_SPACE + args.sku.len() + args.name.len(),
        seeds = [b"inventory", restaurant.key().as_ref(), args.sku.as_ref()],
        bump,
    )] 
    pub item: Account<'info, InventoryItem>,

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

impl<'info> ManageInventory<'info> {
    pub fn add_inventory(&mut self, category: InventoryCategoryType, args: InventoryArgs, bump: u8) -> Result<()> {
        self.item.set_inner(InventoryItem {
            sku: args.sku,
            category,
            name: args.name,
            price: args.price,
            stock: args.stock,
            last_order: Clock::get()?.unix_timestamp,
            initialized: true,
            bump,
        });

        Ok(())
    }

    pub fn update_inventory(&mut self, args: InventoryArgs) -> Result<()> {
        self.item.stock = args.stock;
        self.item.price = args.price;
        self.item.last_order = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

pub fn handler(ctx: Context<ManageInventory>, args: InventoryArgs) -> Result<()> {
    let category = InventoryCategoryType::from_u8(args.category)
        .ok_or(SetupError::InvalidObjectType)?;

    if args.initialized {
        ctx.accounts.update_inventory(args)
    } else {
        ctx.accounts.add_inventory(category, args, ctx.bumps.item)
    }
}

// Implement this method in the InventoryCategoryType enum
impl InventoryCategoryType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::PaperGoods),
            1 => Some(Self::CleaningSupplies),
            2 => Some(Self::Food),
            3 => Some(Self::Beverages),
            4 => Some(Self::Alcohol),
            5 => Some(Self::Equipment),
            6 => Some(Self::Uniform),
            7 => Some(Self::Marketing),
            8 => Some(Self::Other),
            _ => None,
        }
    }
}