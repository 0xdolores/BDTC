use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid health factor")]
    BelowMinHealthFactor,
    #[msg("Cannot liquidate a healthy account")]
    AboveMinHealthFactor,
}
