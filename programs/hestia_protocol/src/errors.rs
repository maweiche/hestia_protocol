use anchor_lang::error_code;


#[error_code]
pub enum SetupError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
    #[msg("You are already verified!")]
    ProfileAlreadyVerified,
    #[msg("You have a Non-Upgradable Membership Type!")]
    InvalidMembership,
    #[msg("You used an invalid condition")]
    InvalidCondition,
    #[msg("You used an invalid object type")]
    InvalidObjectType,
    #[msg("You used an invalid type")]
    InvalidType,
    #[msg("You cannot remove the primary admin")]
    CannotRemovePrimaryAdmin,
    #[msg("Employee does not belong here")]
    EmployeeMismatch
}

#[error_code]
pub enum ProtocolError {
    #[msg("The Protocol is locked, you can't perform this action")]
    ProtocolLocked,
    #[msg("You are not authorized to perform this action")]
    UnauthorizedAdmin,
    #[msg("Airdrop instructions not correct")]
    InstructionsNotCorrect,
    #[msg("Invalid Sale Time")]
    InvalidSaleTime,
    #[msg("Invalid Max Supply")]
    InvalidMaxSupply,
    #[msg("Invalid Price")]
    InvalidPrice,
    #[msg("Mint Count did not increment")]
    InvalidMintCount,
    #[msg("Invalid Balance of Token Pre Mint")]
    InvalidBalancePreMint,
    #[msg("Invalid Balance of Token Post Mint")]
    InvalidBalancePostMint,
    #[msg("Total Supply not increased")]
    TotalSupplyNotIncreased,
    #[msg("Invalid balance pre burn")]
    InvalidBalancePreBurn,
    #[msg("Invalid balance post burn")]
    InvalidBalancePostBurn,
}

#[error_code]
pub enum BuyingError {
    #[msg("You already bought more than 500$ worth of fraction, to buy more you need to do KYC")]
    NotVerified,
    #[msg("Listing is not Live yet, come back later!")]
    NotTimeYet,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
    #[msg("The amount offered does not match the initial token price")]
    PriceMismatch,
    #[msg("Signature authority mismatch")]
    SignatureAuthorityMismatch,
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Insufficient Points")]
    InsufficientPoints
}