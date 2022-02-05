use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum ShihonError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Expected Amount Mismatch
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
}

impl From<ShihonError> for ProgramError {
    fn from(e: ShihonError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
