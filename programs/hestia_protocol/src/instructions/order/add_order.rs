use anchor_lang::{
    prelude::*, 
    solana_program::sysvar::instructions::{
        self,
        load_current_index_checked,
        load_instruction_at_checked
    }
    
};
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, TokenAccount, Token, transfer, Transfer}
};
use std::str::FromStr;
use crate::{
    state::{MenuItem, Customer, CustomerOrder, Restaurant, StatusType, Manager, RewardVoucher}, 
    errors::BuyingError,
    constants::{
        // protocol_currency, 
        signing_authority, 
        ED25519_PROGRAM_ID
    },
};

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    fetch_plugin,
    accounts::{BaseAssetV1, BaseCollectionV1},
    types::{UpdateAuthority, Plugin, FreezeDelegate, PluginType, Royalties},
    instructions::{UpdatePluginV1CpiBuilder, RemovePluginV1CpiBuilder, TransferV1CpiBuilder, BurnV1CpiBuilder}
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CustomerOrderArgs {
    order_id: u64,
    customer: Pubkey,
    customer_name: Option<String>,
    items: Vec<u64>,
    total: u64,
    status: u8,
    created_at: i64,
    updated_at: Option<i64>,
    use_reward: bool,
    bump: u8
}

#[derive(Accounts)]
#[instruction(args: CustomerOrderArgs)]
pub struct AddCustomerOrder<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = CustomerOrder::INIT_SPACE,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref()],
        bump,
    )] 
    pub order: Account<'info, CustomerOrder>,
    #[account(
        init_if_needed,
        payer = signer,
        space = Customer::INIT_SPACE,
        seeds = [b"customer", restaurant.key().as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub customer: Account<'info, Customer>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub currency: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = currency,
        associated_token::authority = signer,
    )]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = currency,
        associated_token::authority = restaurant,
    )]
    pub restaurant_ata: Account<'info, TokenAccount>,
    #[account(mut)] 
    pub restaurant: Account<'info, Restaurant>,
    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
    #[account(constraint = reward.update_authority == manager.key())] 
    pub reward: Option<Account<'info, BaseCollectionV1>>,
    #[account(
        seeds = [b"voucher", reward.as_ref().unwrap().key().as_ref()],
        bump,
    )] 
    pub voucher: Option<Account<'info, RewardVoucher>>,
    #[account(mut)] 
    pub customer_voucher: Option<Signer<'info>>,
    #[account(
        seeds = [b"item", voucher.as_ref().unwrap().item_sku.to_le_bytes().as_ref()],
        bump,
    )] 
    pub menu_item: Account<'info, MenuItem>,
    #[account(address = instructions::ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This account will be checked by the constraint
    pub mpl_core_program: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddCustomerOrder<'info> {
    pub fn add_order(&mut self, args: CustomerOrderArgs, customer_bump: u8, order_bump: u8) -> Result<()> {
        // If the customer is using a reward, then the reward will be burned
        if args.use_reward {
            // require that voucher.item_sku is = menu_item.sku
            // require!(
            //     self.voucher.as_ref().unwrap().item_sku == self.item.sku.parse::<u64>().unwrap(),
            //     BuyingError::InvalidRewardItem
            // );
            
            let signer_seeds: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];

            BurnV1CpiBuilder::new(&self.mpl_core_program.as_ref().unwrap().to_account_info())
            .asset(&self.customer_voucher.as_ref().unwrap().to_account_info())
            .collection(Some(&self.reward.as_ref().unwrap().to_account_info()))
            .payer(&self.signer.to_account_info())
            .authority(Some(&self.manager.to_account_info()))
            .invoke_signed(&[signer_seeds])?;

        };

        if !self.customer.initialized {
            self.customer.set_inner(
                Customer {
                    initialized: true,
                    name: args.customer_name.unwrap_or("no name".to_string()),
                    restaurant: self.restaurant.key(),
                    member_since: Clock::get()?.unix_timestamp - 20 * 60 * 60,
                    total_orders: 1,
                    reward_points: 0,
                    bump: customer_bump
                }
            );
        }

        self.order.set_inner(
            CustomerOrder {
                order_id: args.order_id,
                customer: args.customer,
                items: args.items,
                total: args.total,
                status: StatusType::Pending,
                created_at: Clock::get()?.unix_timestamp - 20 * 60 * 60, // the - 20 * 60 * 60 is to account for the 20 hours difference between the unix timestamp and the actual time
                updated_at: None,
                bump: order_bump
            }
        );

       Ok(())
    }

    pub fn pay_order(&mut self, args: &CustomerOrderArgs, balance_due: u64) -> Result<()> {

        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.signer_ata.to_account_info(),
                    to: self.restaurant_ata.to_account_info(),
                    authority: self.signer.to_account_info(),
                }
            ),
            balance_due * 10u64.pow(self.currency.decimals as u32),
        )?;

        Ok(())
    }

    pub fn stripe_payment(&mut self, current_index: usize, args:& CustomerOrderArgs, balance_due: u64) -> Result<()> {
        let ixs = self.instructions.to_account_info();

        if let Ok(signature_ix) = load_instruction_at_checked(current_index - 1, &ixs) {
            if Pubkey::from_str(ED25519_PROGRAM_ID).unwrap() == signature_ix.program_id {
                require!(
                    signing_authority::ID.to_bytes().eq(&signature_ix.data[16..48]),
                    BuyingError::SignatureAuthorityMismatch
                );

                let mut message_data: [u8; 4] = [0u8; 4];
                message_data.copy_from_slice(&signature_ix.data[112..116]);
                let amount = u32::from_le_bytes(message_data);

                msg!("The message from Signature instruction is: {}", amount);

                let amount_paid = amount as u64;

                require!(
                    amount_paid <= balance_due * 10u64.pow(self.currency.decimals as u32),
                    BuyingError::PriceMismatch
                );
            } else {
                return Err(BuyingError::InvalidInstruction.into());
            }
        } else {
            return Err(BuyingError::InvalidInstruction.into());
        }

        Ok(())
    }
}

pub fn handler(ctx: Context<AddCustomerOrder>, _args: CustomerOrderArgs) -> Result<()> {
    let bump = ctx.bumps.order;
    let args = _args;
    // Instruction Check
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    let mut balance_due = args.total;
    // If the customer is using a reward, then the reward will be burned
    if args.use_reward {
        // require that voucher.item_sku is = menu_item.sku
        // require!(
        //     self.voucher.as_ref().unwrap().item_sku == self.item.sku.parse::<u64>().unwrap(),
        //     BuyingError::InvalidRewardItem
        // );

        let menu_item_price = ctx.accounts.menu_item.price; 

        // subtract the price of the menu item from the total
        balance_due -= menu_item_price; 
    };

    // If the current index is 0, then the buyer must pay the fraction via the listing currency, else it's a stripe payment
    match current_index {
        0 => ctx.accounts.pay_order(&args, balance_due)?,
        _ => ctx.accounts.stripe_payment(current_index, &args, balance_due)?
    }

    ctx.accounts.add_order(args, ctx.accounts.customer.bump, bump)?;

    Ok(())
}