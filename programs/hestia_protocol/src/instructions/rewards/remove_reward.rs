use anchor_lang::prelude::*;
use crate::state::{AdminProfile, Manager, Restaurant, RewardVoucher};

use mpl_core::accounts::BaseCollectionV1;

#[derive(Accounts)]
pub struct RemoveRewardVoucher<'info> {
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
    #[account(constraint = reward.update_authority == manager.key())] 
    pub reward: Account<'info, BaseCollectionV1>,
    #[account(
        mut,
        close = restaurant_admin,
        seeds = [b"voucher", reward.key().as_ref()],
        bump,
    )] 
    pub voucher: Account<'info, RewardVoucher>,
    pub system_program: Program<'info, System>,
}

impl<'info> RemoveRewardVoucher<'info> {
    pub fn remove_reward_voucher(&mut self) -> Result<()> {
       Ok(())
    } 
}

pub fn handler(ctx: Context<RemoveRewardVoucher>) -> Result<()> {
    ctx.accounts.remove_reward_voucher()?;
    Ok(())
}