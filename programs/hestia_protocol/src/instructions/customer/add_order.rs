use anchor_lang::{
    prelude::*, 
    solana_program::{
        program_memory::sol_memcpy,
        sysvar::instructions::{
            self,
            load_current_index_checked,
            load_instruction_at_checked
        }
    }
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, TokenAccount, Token, transfer, Transfer}
};
use std::str::FromStr;
use anchor_spl::associated_token::Create;
use mpl_core::accounts::BaseCollectionV1;
use crate::{state::{Customer, CustomerOrder, Restaurant, StatusType, EmployeeType, InventoryCategoryType, InventoryItem}, errors::{SetupError, BuyingError},
constants::{
    // protocol_currency, 
    signing_authority, 
    ED25519_PROGRAM_ID
},
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
    bump: u8
}

#[derive(Accounts)]
#[instruction(args: CustomerOrderArgs)]
pub struct AddCustomerOrder<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = CustomerOrder::INIT_SPACE,
        seeds = [b"order", restaurant.key().as_ref(), args.order_id.to_le_bytes().as_ref(), signer.key().as_ref()],
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
    #[account(address = instructions::ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddCustomerOrder<'info> {
    pub fn add_order(&mut self, args: CustomerOrderArgs, customer_bump: u8, order_bump: u8) -> Result<()> {

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

    pub fn pay_order(&mut self, args: &CustomerOrderArgs) -> Result<()> {

        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.signer_ata.to_account_info(),
                    to: self.restaurant_ata.to_account_info(),
                    authority: self.signer.to_account_info(),
                }
            ),
            args.total * 10u64.pow(self.currency.decimals as u32),
        )?;

        Ok(())
    }

    pub fn stripe_payment(&mut self, current_index: usize, args:& CustomerOrderArgs) -> Result<()> {
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
                    amount_paid <= args.total * 10u64.pow(self.currency.decimals as u32),
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

    // If the current index is 0, then the buyer must pay the fraction via the listing currency, else it's a stripe payment
    match current_index {
        0 => ctx.accounts.pay_order(&args)?,
        _ => ctx.accounts.stripe_payment(current_index, &args)?
    }

    ctx.accounts.add_order(args, ctx.accounts.customer.bump, bump)?;

    Ok(())
}