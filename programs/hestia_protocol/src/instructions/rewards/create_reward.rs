// // CREATE REWARD ESTABLISHES A "REWARD" ACCOUNT
// // THIS THEN ALLOWS THE RESTAURANT OWNER TO THEN "ADD" REWARD
// // WHICH WHAT THE USERS WILL MINT REWARDS FROM

// use anchor_lang::prelude::*;
// use mpl_core::{
//     instructions::CreateCollectionV2CpiBuilder, 
//     types::{
//         Attribute, Attributes, Creator, ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, OracleInitInfo, PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet, ValidationResultsOffset
//     }, 
//     ID as MPL_CORE_PROGRAM_ID
// };

// use crate::state::{AdminProfile, Manager, Protocol, Restaurant};

// #[derive(AnchorDeserialize, AnchorSerialize)]
// pub struct CreateRewardArgs {
//     pub name: String,
//     pub uri: String,
//     pub sku: String,
//     pub category: u8,
// }

// #[derive(Accounts)]
// #[instruction(args: CreateRewardArgs)]
// pub struct CreateReward<'info> {
//     #[account(
//         mut,
//         seeds = [b"reward", args.sku.as_bytes().as_ref(), restaurant.key().as_ref()],
//         bump
//     )] 
//     /// CHECK: This account will be checked by the constraint
//     pub reward: UncheckedAccount<'info>,
//     #[account(mut)]
//     pub restaurant_admin: Signer<'info>,
//     #[account(
//         seeds = [b"admin", restaurant_admin.key().as_ref()],
//         bump
//     )]
//     pub admin_profile: Account<'info, AdminProfile>,
//     #[account(
//         constraint = restaurant.owner == *restaurant_admin.key,
//     )] 
//     pub restaurant: Account<'info, Restaurant>,
//     #[account(
//         seeds = [b"manager"],
//         bump = manager.bump,
//     )]
//     pub manager: Account<'info, Manager>,
//     #[account(
//         seeds = [b"protocol"],
//         bump = protocol.bump,
//     )]
//     pub protocol: Account<'info, Protocol>,
//     #[account(address = MPL_CORE_PROGRAM_ID)]
//     /// CHECK: This account will be checked by the constraint
//     pub mpl_core_program: UncheckedAccount<'info>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> CreateReward<'info> {
//     pub fn create_reward(&mut self, args: CreateRewardArgs, bump: u8) -> Result<()> {
//         let reference = args.sku.clone();
//         let name = args.name;
//         // Add an Attribute Plugin that will hold the event details
//         let mut collection_plugin: Vec<PluginAuthorityPair> = vec![];

//         let attribute_list: Vec<Attribute> = vec![
//             Attribute { key: "Sku".to_string(), value: args.sku },
//             Attribute { key: "Name".to_string(), value: name.clone() },
//             Attribute { key: "Category".to_string(), value: args.category.to_string() },
//             Attribute { key: "Restaurant".to_string(), value: self.restaurant.name.to_string() },
//         ];
//         collection_plugin.push(PluginAuthorityPair { plugin: Plugin::Attributes(Attributes { attribute_list }), authority: Some(PluginAuthority::UpdateAuthority) });
//         collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentBurnDelegate( PermanentBurnDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) });
//         collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentFreezeDelegate( PermanentFreezeDelegate { frozen: false }), authority: Some(PluginAuthority::UpdateAuthority) });
//         collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentTransferDelegate( PermanentTransferDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) });
//         collection_plugin.push(PluginAuthorityPair { plugin: Plugin::Royalties( Royalties { basis_points: 200, creators: vec![Creator {address: self.manager.key(), percentage: 100 }], rule_set: RuleSet::None}), authority: Some(PluginAuthority::UpdateAuthority) });
        
//         // Add an External Plugin Adapter that will hold the event details
//         let mut collection_external_plugin: Vec<ExternalPluginAdapterInitInfo> = vec![];
        
//         collection_external_plugin.push(ExternalPluginAdapterInitInfo::Oracle(
//             OracleInitInfo {
//                 base_address: self.protocol.key(),
//                 base_address_config: None,
//                 results_offset: Some(ValidationResultsOffset::Anchor),
//                 lifecycle_checks: vec![
//                     (HookableLifecycleEvent::Transfer, ExternalCheckResult { flags: 4 }),
//                     (HookableLifecycleEvent::Burn, ExternalCheckResult { flags: 4 }),
//                     (HookableLifecycleEvent::Update, ExternalCheckResult { flags: 4 }),
//                     (HookableLifecycleEvent::Create, ExternalCheckResult { flags: 4 }),
//                 ],
//                 init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
//             }
//         ));
//         let restaurant_key = self.restaurant.key();
//         let manager_seed: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];
//         let reward_seed = &[
//             b"reward", 
//             reference.as_bytes().as_ref(), 
//             restaurant_key.as_ref(), 
//             &[bump]
//         ];

//         // Create the Collection that will hold the watch 
//         CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
//         .collection(&self.reward.to_account_info())
//         .update_authority(Some(&self.manager.to_account_info()))
//         .payer(&self.restaurant_admin.to_account_info())
//         .system_program(&self.system_program.to_account_info())
//         .name(name.clone())
//         .uri(args.uri)
//         .plugins(collection_plugin)
//         .external_plugin_adapters(collection_external_plugin)
//         .add_remaining_account(&self.protocol.to_account_info(), false, false)
//         .invoke_signed(&[manager_seed, reward_seed])?;

//         Ok(())
//     }
// }

// pub fn handler(ctx: Context<CreateReward>, args: CreateRewardArgs) -> Result<()> {
//     ctx.accounts.create_reward(args, ctx.bumps.reward)?;

//     Ok(())
// }

use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder, 
    types::{
        Attribute, Attributes, Creator, ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, OracleInitInfo, PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet, ValidationResultsOffset
    }, 
    ID as MPL_CORE_PROGRAM_ID
};
use crate::state::{AdminProfile, Manager, Protocol, Restaurant};
use crate::errors::RewardError;

/*
    Create Reward Instruction

    Functionality:
    - Establishes a "Reward" account for a restaurant
    - Sets up various plugins and attributes for the reward
    - Creates a collection using the MPL Core program

    Security checks:
    - Ensures the signer is the restaurant admin
    - Verifies that the restaurant belongs to the admin
*/

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateRewardArgs {
    pub name: String,
    pub uri: String,
    pub sku: String,
    pub category: u8,
}

#[derive(Accounts)]
#[instruction(args: CreateRewardArgs)]
pub struct CreateReward<'info> {
    #[account(
        mut,
        seeds = [b"reward", args.sku.as_bytes(), restaurant.key().as_ref()],
        bump
    )] 
    /// CHECK: This account will be initialized by the MPL Core program
    pub reward: UncheckedAccount<'info>,

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

    #[account(
        seeds = [b"protocol"],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This account is checked by the constraint
    pub mpl_core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateReward<'info> {
    pub fn create_reward(&self, args: CreateRewardArgs, bump: u8) -> Result<()> {
        let collection_plugins = self.create_collection_plugins(&args);
        let collection_external_plugins = self.create_external_plugins();

        self.create_collection(args, bump, collection_plugins, collection_external_plugins)?;

        Ok(())
    }

    fn create_collection_plugins(&self, args: &CreateRewardArgs) -> Vec<PluginAuthorityPair> {
        vec![
            PluginAuthorityPair { 
                plugin: Plugin::Attributes(Attributes { 
                    attribute_list: vec![
                        Attribute { key: "Sku".to_string(), value: args.sku.clone() },
                        Attribute { key: "Name".to_string(), value: args.name.clone() },
                        Attribute { key: "Category".to_string(), value: args.category.to_string() },
                        Attribute { key: "Restaurant".to_string(), value: self.restaurant.name.to_string() },
                    ] 
                }),
                authority: Some(PluginAuthority::UpdateAuthority) 
            },
            PluginAuthorityPair { plugin: Plugin::PermanentBurnDelegate(PermanentBurnDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) },
            PluginAuthorityPair { plugin: Plugin::PermanentFreezeDelegate(PermanentFreezeDelegate { frozen: false }), authority: Some(PluginAuthority::UpdateAuthority) },
            PluginAuthorityPair { plugin: Plugin::PermanentTransferDelegate(PermanentTransferDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) },
            PluginAuthorityPair { 
                plugin: Plugin::Royalties(Royalties { 
                    basis_points: 200, 
                    creators: vec![Creator { address: self.manager.key(), percentage: 100 }], 
                    rule_set: RuleSet::None
                }),
                authority: Some(PluginAuthority::UpdateAuthority) 
            },
        ]
    }

    fn create_external_plugins(&self) -> Vec<ExternalPluginAdapterInitInfo> {
        vec![ExternalPluginAdapterInitInfo::Oracle(
            OracleInitInfo {
                base_address: self.protocol.key(),
                base_address_config: None,
                results_offset: Some(ValidationResultsOffset::Anchor),
                lifecycle_checks: vec![
                    (HookableLifecycleEvent::Transfer, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Burn, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Update, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Create, ExternalCheckResult { flags: 4 }),
                ],
                init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
            }
        )]
    }

    fn create_collection(&self, args: CreateRewardArgs, bump: u8, plugins: Vec<PluginAuthorityPair>, external_plugins: Vec<ExternalPluginAdapterInitInfo>) -> Result<()> {
        let manager_seed: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];
        let reference = args.sku.clone();
        let restaurant_key = self.restaurant.key();
        let reward_seed = &[
            b"reward", 
            reference.as_bytes().as_ref(), 
            restaurant_key.as_ref(), 
            &[bump]
        ];

        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.reward.to_account_info())
            .update_authority(Some(&self.manager.to_account_info()))
            .payer(&self.restaurant_admin.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(plugins)
            .external_plugin_adapters(external_plugins)
            .add_remaining_account(&self.protocol.to_account_info(), false, false)
            .invoke_signed(&[manager_seed, reward_seed])?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateReward>, args: CreateRewardArgs) -> Result<()> {
    ctx.accounts.create_reward(args, ctx.bumps.reward)
}