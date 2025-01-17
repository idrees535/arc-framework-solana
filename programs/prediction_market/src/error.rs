use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("At least one outcome is required")]
    NoOutcomes,
    #[msg("Liquidity parameter b must be greater than zero")]
    InvalidB,
    #[msg("Duration must be positive")]
    InvalidDuration,
    #[msg("Invalid owner for the mint account.")]
    InvalidOwner,
    #[msg("Invalid mint account.")]
    InvalidMint,
    #[msg("Market is closed")]
    MarketClosed,
    #[msg("Invalid outcome index")]
    InvalidOutcome,
    #[msg("Must buy at least one share")]
    InvalidShares,
    #[msg("Overflow occurred")]
    Overflow,
    #[msg("Underflow occurred")]
    Underflow,
    #[msg("Math error")]
    MathError,
    #[msg("Market not closed yet")]
    MarketNotClosed,
    #[msg("Market already settled")]
    MarketAlreadySettled,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("No fees to withdraw")]
    NoFeesToWithdraw,
    #[msg("No shares to claim")]
    NoSharesToClaim,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("make it short assholde")]
    OutcomeNameTooLong,
    #[msg("This is not fucking acceptable")]
    InvalidMintKey,
    #[msg("Go, get them first")]
    InsufficientShares,
    #[msg("Baz aa ja tou bahi")]
    InvalidAccounts,
    #[msg("Tou b madarchod")]
    InvalidMintAuthority,
    #[msg("Tou b madarchod")]
    MintAlreadyInitialized,
    

    #[msg("Market is already closed")]
    MarketAlreadyClosed,
    #[msg("Market end time has not yet passed")]
    MarketNotExpired,

    #[msg("Market is not settled yet")]
    MarketNotSettled,
    #[msg("outcome mint doesn/t match with outcome_index mint address")]
    InvalidOutcomeMint,
    #[msg("User share account is invalid")]
    InvalidUserShareAccount,
    #[msg("No remaining funds")]
    NoRemainingFunds,
    #[msg("No remaining funds")]
    InvalidInput,
    #[msg("No remaining funds")]
    InvalidLiquidityParameter,
      #[msg("The initial funds provided are insufficient for this market.")]
    InsufficientInitialFunds,
 #[msg("Freeze authority for outcome mint is not disabled")]
    FreezeAuthorityNotDisabled

  

}
