//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Candidate instruction
pub fn process_candidate(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // The previously created CandidateLimitRecord has a mechanism to limit the number of coins and people called candidate limit bar
    // CandidateLimitRecord is issued for each new candidate, always referring to the previous record to calculate the number of people and the total coin limit
    // Note: The tanistry will not be formed until the last candidate arrives
    // Note: If you want to know what happens next...Look at state/tanistry.rs
}
