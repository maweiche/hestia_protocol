use anchor_lang::{
    prelude::*, 
    solana_program::program_memory::sol_memcpy,
};
use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    accounts::BaseCollectionV1,
    instructions::CreateV1CpiBuilder
};
use crate::{
    state::{RewardVoucher, CompletedRewardVoucher, Customer, Restaurant, Manager, Protocol},
    errors::BuyingError
};

/*
    Buy Reward Voucher Instruction

    Functionality:
    - Allows a customer to purchase a reward voucher using their reward points.
    - Creates a new asset (voucher) for the customer using the MPL Core program.
    - Updates the voucher state (completes it if all shares are sold).

    Security checks:
    - Ensures the customer has enough reward points to purchase the voucher.
    - Verifies that the reward's update authority matches the manager's key.
*/

#[derive(Accounts)]
pub struct BuyRewardVoucher<'info> {
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
    #[account(
        seeds = [b"customer", restaurant.key().as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub customer: Account<'info, Customer>,
    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(constraint = reward.update_authority == manager.key())] 
    pub reward: Account<'info, BaseCollectionV1>,
    #[account(
        seeds = [b"voucher", reward.key().as_ref()],
        bump,
    )] 
    pub voucher: Account<'info, RewardVoucher>,
    #[account(mut)] 
    pub customer_voucher: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This account will be checked by the MPL Core program
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyRewardVoucher<'info> {
    pub fn buy_reward(&mut self, uri: String) -> Result<()> {
        // Check if customer has enough points
        require!(
            self.voucher.price <= self.customer.reward_points,
            BuyingError::InsufficientPoints
        );

        // Deduct points from customer
        self.customer.reward_points -= self.voucher.price;

        // Create the voucher asset
        self.create_voucher_asset(&uri)?;

        // Update voucher state
        self.update_voucher_state()?;

        Ok(())
    }

    fn create_voucher_asset(&self, uri: &str) -> Result<()> {
        let signer_seeds: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];

        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.customer_voucher.to_account_info())
            .collection(Some(&self.reward.to_account_info()))
            .authority(Some(&self.manager.to_account_info()))
            .payer(&self.payer.to_account_info())
            .owner(Some(&self.signer.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(format!("{} - {}", self.reward.name, self.voucher.id))
            .uri(uri.to_string())
            .add_remaining_account(&self.protocol.to_account_info(), false, false)
            .invoke_signed(&[signer_seeds])?;

        Ok(())
    }

    fn update_voucher_state(&mut self) -> Result<()> {
        if self.voucher.share_sold + 1 == self.voucher.share {
            self.complete_voucher()?;
        } else {
            self.voucher.share_sold += 1;
        }
        Ok(())
    }

    fn complete_voucher(&self) -> Result<()> {
        let info = self.voucher.to_account_info(); 
        let mut data = info.try_borrow_mut_data()?;

        let completed_reward_voucher = CompletedRewardVoucher {
            id: self.voucher.id,
            category: self.voucher.category.clone(),
            reward: self.voucher.reward,
            share: self.voucher.share,
            price: self.voucher.price,
            bump: self.voucher.bump,
        };

        let mut writer: Vec<u8> = Vec::with_capacity(CompletedRewardVoucher::INIT_SPACE);
        completed_reward_voucher.try_serialize(&mut writer)?;
        writer.truncate(CompletedRewardVoucher::INIT_SPACE);

        sol_memcpy(&mut data, &writer, writer.len());
        Ok(())
    }
}

pub fn handler(ctx: Context<BuyRewardVoucher>, uri: String) -> Result<()> {
    ctx.accounts.buy_reward(uri)
}