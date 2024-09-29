use anchor_lang::prelude::*;
use crate::state::{AdminProfile, Manager, Restaurant, RewardVoucher};
use crate::errors::RewardError;
use mpl_core::accounts::BaseCollectionV1;

/*
    Remove Reward Voucher Instruction

    Functionality:
    - Removes a reward voucher from the restaurant's reward system
    - Closes the RewardVoucher account and returns the rent to the restaurant admin

    Security checks:
    - Ensures the signer is the restaurant admin
    - Verifies that the restaurant belongs to the admin
    - Checks that the reward's update authority is the manager
*/

#[derive(Accounts)]
pub struct RemoveRewardVoucher<'info> {
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
        mut,
        close = restaurant_admin,
        seeds = [b"voucher", reward.key().as_ref()],
        bump = voucher.bump,
    )] 
    pub voucher: Account<'info, RewardVoucher>,

    pub system_program: Program<'info, System>,
}

impl<'info> RemoveRewardVoucher<'info> {
    pub fn remove_reward_voucher(&mut self) -> Result<()> {
        // The actual removal is handled by the `close = restaurant_admin` constraint
        // We can perform any additional cleanup or logging here if needed
        emit!(RewardVoucherRemoved {
            voucher_id: self.voucher.id,
            restaurant: self.restaurant.key(),
            admin: self.restaurant_admin.key(),
        });

        Ok(())
    } 
}

pub fn handler(ctx: Context<RemoveRewardVoucher>) -> Result<()> {
    ctx.accounts.remove_reward_voucher()
}

// Event emitted when a reward voucher is removed
#[event]
pub struct RewardVoucherRemoved {
    pub voucher_id: u64,
    pub restaurant: Pubkey,
    pub admin: Pubkey,
}