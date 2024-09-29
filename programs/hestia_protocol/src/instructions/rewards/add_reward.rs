use anchor_lang::prelude::*;
use crate::{
    errors::RewardError,
    state::{AdminProfile, Manager, MenuCategoryType, Restaurant, RewardVoucher}
};
use mpl_core::accounts::BaseCollectionV1;

/*
    Add Reward Voucher Instruction

    Functionality:
    - Creates a new RewardVoucher account for a specific reward
    - Links the voucher to a menu item and sets its properties

    Security checks:
    - Ensures the signer is the restaurant admin
    - Verifies that the restaurant belongs to the admin
    - Checks that the reward's update authority is the manager
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddRewardVoucherArgs {
    pub id: u64,
    pub item_sku: u64,
    pub reward: Pubkey,
    pub category: u8,
    pub share: u16,
    pub price: u64,
    pub starting_time: i64,
}

#[derive(Accounts)]
#[instruction(args: AddRewardVoucherArgs)]
pub struct AddRewardVoucher<'info> {
    #[account(mut)]
    pub restaurant_admin: Signer<'info>,

    #[account(
        seeds = [b"admin", restaurant_admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        constraint = restaurant.owner == restaurant_admin.key() @ RewardError::Unauthorized,
    )] 
    pub restaurant: Account<'info, Restaurant>,

    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,

    #[account(constraint = reward.update_authority == manager.key() @ RewardError::InvalidRewardAuthority)] 
    pub reward: Account<'info, BaseCollectionV1>,

    #[account(
        init,
        payer = restaurant_admin,
        space = RewardVoucher::INIT_SPACE,
        seeds = [b"voucher", reward.key().as_ref()],
        bump,
    )] 
    pub voucher: Account<'info, RewardVoucher>,

    pub system_program: Program<'info, System>,
}

impl<'info> AddRewardVoucher<'info> {
    pub fn add_reward_voucher(&mut self, args: AddRewardVoucherArgs, bump: u8) -> Result<()> {
        let category = MenuCategoryType::from_u8(args.category)
            .ok_or(RewardError::InvalidCategory)?;

        self.voucher.set_inner(RewardVoucher {
            id: args.id,
            item_sku: args.item_sku,
            reward: self.reward.key(),
            restaurant: self.restaurant.key(),
            category,
            share: args.share,
            share_sold: 0, 
            price: args.price,
            starting_time: args.starting_time,
            bump,
        });
        
        Ok(())
    } 
}

pub fn handler(ctx: Context<AddRewardVoucher>, args: AddRewardVoucherArgs) -> Result<()> {
    ctx.accounts.add_reward_voucher(args, ctx.bumps.voucher)
}

impl MenuCategoryType {
    fn from_u8(value: u8) -> Option<Self> {
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