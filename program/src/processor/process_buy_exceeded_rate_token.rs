//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Buy instruction
pub fn process_buy_exceeded_rate_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    unimplemented!();
    // Almost the same as methods that CandidateLimitRecord has done with by using OutsideBuyerRecord
    // we need to set time limits for this functions in some way.
}
