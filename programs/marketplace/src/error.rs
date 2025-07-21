use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Custom error message")]
    NameTooLong,
}
