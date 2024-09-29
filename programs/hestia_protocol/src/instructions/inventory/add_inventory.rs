use anchor_lang::prelude::*;
use crate::{state::{AdminProfile, Restaurant, InventoryCategoryType, InventoryItem}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddInventoryArgs {
    sku: String,
    category: u8,
    name: String,
    price: u64,
    stock: u64,
    last_order: i64,
    initialized: bool,
    bump: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct DeductStockArgs {
    sku: u64,
    amount: u64,
}

#[derive(Accounts)]
#[instruction(args: AddInventoryArgs)]
pub struct AddInventory<'info> {
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
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        constraint = restaurant.owner == *restaurant_admin.key,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddInventory<'info> {
    pub fn add_inventory(&mut self, category: InventoryCategoryType, args: AddInventoryArgs, bump: u8) -> Result<()> {

        self.item.set_inner(
            InventoryItem {
                sku: args.sku,
                category,
                name: args.name,
                price: args.price,
                stock: args.stock,
                last_order: Clock::get()?.unix_timestamp - 20 * 60 * 60,
                initialized: true,
                bump
            }
        );

       Ok(())
    }

    pub fn update_inventory(&mut self, args: AddInventoryArgs) -> Result<()> {
        self.item.stock = args.stock;
        self.item.price = args.price;

        Ok(())
    }

}

pub fn handler(ctx: Context<AddInventory>, args: AddInventoryArgs) -> Result<()> {
    let bump = ctx.bumps.item;

    let object_type = match args.category{
        0 => InventoryCategoryType::PaperGoods,
        1 => InventoryCategoryType::CleaningSupplies,
        2 => InventoryCategoryType::Food,
        3 => InventoryCategoryType::Beverages,
        4 => InventoryCategoryType::Alcohol,
        5 => InventoryCategoryType::Equipment,
        6 => InventoryCategoryType::Uniform,
        7 => InventoryCategoryType::Marketing,
        8 => InventoryCategoryType::Other,
        _ => return Err(SetupError::InvalidObjectType.into())
    };

    if args.initialized {
        ctx.accounts.update_inventory(args)?;
    } else {
        ctx.accounts.add_inventory(object_type, args, bump)?;
    }

    Ok(())
}  