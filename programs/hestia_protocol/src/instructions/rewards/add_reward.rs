use anchor_lang::prelude::*;
use crate::{
    errors::SetupError, state::{AdminProfile, Manager, MenuCategoryType, Restaurant, RewardVoucher}
};
use mpl_core::accounts::BaseCollectionV1;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct AddRewardVoucherArgs {
    pub id: u64,
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
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        constraint = restaurant.owner == *restaurant_admin.key,
    )] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
    #[account(mut)]
    ///CHECK: This account will be checked by the constraint
    pub creator: AccountInfo<'info>,
    #[account(constraint = reward.update_authority == manager.key())] 
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
    pub fn add_reward_voucher(
        &mut self, 
        id: u64, 
        price: u64,
        share: u16,
        category: MenuCategoryType, 
        starting_time: i64,
        bump: u8
    ) -> Result<()> {

        self.voucher.set_inner(
            RewardVoucher {
                id,
                reward: self.reward.key(),
                restaurant: self.restaurant.key(),
                category,
                share,
                share_sold: 0, 
                price,
                starting_time,
                bump,
            }
        );
        
       Ok(())
    } 
}

pub fn handler(ctx: Context<AddRewardVoucher>, args: AddRewardVoucherArgs) -> Result<()> {
    let bump = ctx.bumps.voucher;

    let category = match args.category {
        0 => MenuCategoryType::Combo,
        1 => MenuCategoryType::Side,
        2 => MenuCategoryType::Entree,
        3 => MenuCategoryType::Dessert,
        4 => MenuCategoryType::Beverage,
        5 => MenuCategoryType::Alcohol,
        6 => MenuCategoryType::Other,
        _ => return Err(SetupError::InvalidType.into()),
    };

    ctx.accounts.add_reward_voucher(
        args.id, 
        args.price, 
        args.share, 
        category, 
        args.starting_time, 
        bump
    )?;

    Ok(())
}