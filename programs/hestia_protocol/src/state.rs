use anchor_lang::prelude::*;
use mpl_core::types::OracleValidation;

/// Protocol-level Structures

#[account]
pub struct Protocol {
    pub validation: OracleValidation,
    pub bump: u8,
}

impl Space for Protocol {
    const INIT_SPACE: usize = 8 + 5 + 1;
}

#[account]
pub struct Manager {
    pub bump: u8,
}

impl Space for Manager {
    const INIT_SPACE: usize = 8 + 1;
}

#[account]
pub struct AdminProfile {
    pub username: String,
    pub creation_time: i64,
    pub bump: u8,
}

impl Space for AdminProfile {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8 + 1;
}

/// Restaurant-related Structures

#[account]
pub struct Restaurant {
    pub restaurant_type: RestaurantType,
    pub owner: Pubkey,
    pub name: String,
    pub symbol: String,
    pub currency: Pubkey,
    pub url: String,
    pub customer_count: u64,
    pub bump: u8,
}

impl Space for Restaurant {
    const INIT_SPACE: usize = 8 + 4 + 4 + 32 + 32 + 4 + 32 + 32 + 4 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum RestaurantType {
    Foodtruck,
    Cafe,
    Restaurant,
}

#[account]
pub struct Employee {
    pub wallet: Pubkey,
    pub restaurant: Pubkey,
    pub employee_type: EmployeeType,
    pub username: String,
    pub initialized: bool,
    pub bump: u8,
}

impl Space for Employee {
    const INIT_SPACE: usize = 8 + 32 + 32 + 4 + 4 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace, PartialEq)]
pub enum EmployeeType {
    TeamMember,
    TeamLeader,
    Manager,
    Director,
}

/// Inventory-related Structures

#[account]
pub struct InventoryItem {
    pub sku: String,
    pub category: InventoryCategoryType,
    pub name: String,
    pub price: u64,
    pub stock: u64,
    pub last_order: i64,
    pub initialized: bool,
    pub bump: u8,
}

impl Space for InventoryItem {
    const INIT_SPACE: usize = 8 + 8 + 32 + 4 + 8 + 8 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum InventoryCategoryType {
    PaperGoods,
    CleaningSupplies,
    Food,
    Beverages,
    Alcohol,
    Equipment,
    Uniform,
    Marketing,
    Other,
}

/// Menu-related Structures

#[account]
pub struct Menu {
    pub initialized: bool,
    pub bump: u8,
}

impl Space for Menu {
    const INIT_SPACE: usize = 8 + 4 + 1;
}

#[account]
pub struct MenuItem {
    pub sku: String,
    pub category: MenuCategoryType,
    pub name: String,
    pub price: u64,
    pub description: String,
    pub active: bool,
    pub bump: u8,
}

impl Space for MenuItem {
    const INIT_SPACE: usize = 8 + 4 + MenuCategoryType::INIT_SPACE + 4 + 8 + 4 + 4 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum MenuCategoryType {
    Combo,
    Side,
    Entree,
    Dessert,
    Beverage,
    Alcohol,
    Other,
}

#[account]
pub struct IngredientList {
    pub menu_item: Pubkey,
    pub ingredients: Vec<(Pubkey, u64)>, // (InventoryItem pubkey, quantity)
    pub bump: u8,
}

impl Space for IngredientList {
    const INIT_SPACE: usize = 8 + 32 + 4 + 1;
}

/// Customer-related Structures

#[account]
pub struct Customer {
    pub initialized: bool,
    pub name: String,
    pub restaurant: Pubkey,
    pub member_since: i64,
    pub total_orders: u64,
    pub reward_points: u64,
    pub bump: u8,
}

impl Space for Customer {
    const INIT_SPACE: usize = 8 + 4 + 4 + 32 + 8 + 1;
}

#[account]
pub struct CustomerOrder {
    pub order_id: u64,
    pub customer: Pubkey,
    pub items: Vec<u64>,
    pub total: u64,
    pub status: StatusType,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub bump: u8,
}

impl Space for CustomerOrder {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace, PartialEq)]
pub enum StatusType {
    Pending,
    Completed,
    Finalized,
    Cancelled,
}

/// Reward-related Structures

#[account]
pub struct Reward {
    pub category: MenuCategoryType,
    pub restaurant: Pubkey,
    pub reward_points: u64,
    pub reward_item: Pubkey,
    pub bump: u8,
}

impl Space for Reward {
    const INIT_SPACE: usize = 8 + 32 + 4 + 32 + 4 + 8 + 32 + 4 + 1;
}

#[account]
pub struct RewardVoucher {
    pub id: u64,
    pub item_sku: u64,
    pub reward: Pubkey,
    pub restaurant: Pubkey,
    pub category: MenuCategoryType,
    pub share: u16,
    pub share_sold: u16,
    pub price: u64,
    pub starting_time: i64,
    pub bump: u8,
}

impl Space for RewardVoucher {
    const INIT_SPACE: usize = 8 + 8 + 32 + 32 + MenuCategoryType::INIT_SPACE + 2 + 2 + 8 + 8 + 1;
}

#[account]
pub struct CompletedRewardVoucher {
    pub id: u64,
    pub reward: Pubkey,
    pub category: MenuCategoryType,
    pub share: u16,
    pub price: u64,
    pub bump: u8,
}

impl Space for CompletedRewardVoucher {
    const INIT_SPACE: usize = 8 + 8 + 32 + MenuCategoryType::INIT_SPACE + 2 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Attributes {
    pub key: String,
    pub value: String,
}