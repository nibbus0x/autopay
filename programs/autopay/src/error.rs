use anchor_lang::prelude::*;

#[error_code]
pub enum AutoPayError {
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Context is missing required bump")]
    MissingBump,
    #[msg("Schedule string is missing or invalid")]
    InvalidScheduleString
}
