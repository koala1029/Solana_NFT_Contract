use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient caller")]
    UnauthorizedCaller,
    #[msg("Insufficient collection")]
    UnauthorizedCollection,
    #[msg("Insufficient amount")]
    InsufficientAmount,
}
