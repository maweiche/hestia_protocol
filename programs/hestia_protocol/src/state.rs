use anchor_lang::prelude::*;
use mpl_core::types::OracleValidation;

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
pub struct Admin {
    pub publickey: Pubkey,
    pub username: String,
    pub initialized: i64,
}

impl Space for Admin {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8;
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
    Foodtruck, // 0
    Cafe, // 1
    Restaurant, // 2
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

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum EmployeeType {
    TeamMember, // 0
    TeamLeader, // 1
    Manager, // 2
    Director, // 3
}

#[account]
pub struct InventoryItem {
    pub sku: u64,              // Stock Keeping Unit -- how we identify the product
    pub category: InventoryCategoryType,      // Category of the product -- stored as public key for easy sorting and filtering
    pub name: String,          // Name of the product -- what the product is called
    pub price: u64,            // Price of the product -- how much it costs for ordering
    pub stock: u64,            // Stock of the product -- how many units are available, will be updated as orders are made
    pub last_order: i64,       // Last time the product was ordered -- stored as unix timestamp
    pub initialized: bool,     // Initialized status of the product -- whether it is available for ordering
    pub bump: u8,
}

impl Space for InventoryItem {
    const INIT_SPACE: usize = 8 + 8 + 32 + 4 + 8 + 8 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum InventoryCategoryType {
    PaperGoods, // 0
    CleaningSupplies, // 1
    Food, // 2
    Beverages, // 3
    Alcohol, // 4
    Equipment, // 5
    Uniform, // 6
    Marketing, // 7
    Other, // 8
}

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
    pub sku: u64,              // Stock Keeping Unit -- how we identify the product
    pub category: MenuCategoryType,      // Category of the product -- stored as public key for easy sorting and filtering
    pub name: String,          // Name of the product -- what the product is called
    pub price: u64,            // Price of the product -- how much it costs for ordering
    pub description: String,   // Description of the product -- what the product is
    pub ingredients: Vec<u64>, // Ingredients of the product -- what is used to make the product, this is what will be used to deduct from the inventory
    pub active: bool,          // Active status of the product -- whether it is available for ordering
    pub bump: u8,
}

impl Space for MenuItem {
    const INIT_SPACE: usize = 8 + 8 + 32 + 4 + 8 + 8 + 8 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum MenuCategoryType {
    Combo, // 0
    Side, // 1
    Entree, // 2
    Dessert, // 3
    Beverage, // 4
    Alcohol, // 5
    Other, // 6
}

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
    pub order_id: u64,         // Order ID -- unique identifier for the order
    pub customer: Pubkey,      // Customer of the order -- who made the order
    pub items: Vec<u64>,       // Items in the order -- what products were ordered, skus of the products
    pub total: u64,            // Total of the order -- how much the order costs
    pub status: StatusType,            // Status of the order -- what state the order is in (0: pending, 1: completed, 2: finalized, 3: cancelled)
    pub created_at: i64,       // Created at -- when the order was made, stored as unix timestamp
    pub updated_at: Option<i64>,       // Updated at -- when the order was last updated, stored as unix timestamp
    pub bump: u8,
}

impl Space for CustomerOrder {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8 + 1 + 8 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum StatusType {
    Pending, // 0
    Completed, // 1
    Finalized, // 2
    Cancelled, // 3
}

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

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Attributes {
    pub key: String,
    pub value: String,
}