//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Bump instruction
pub fn process_bump_self_rate(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // what this func needed is only some validators
    // for update amount of self rating as coin on CandidateLimitRecord, the candidate need to issue same bcToken again without metadata. if want stay his rate same, bcToken with 0 deposit be permitted to issue. and we need to verify that it is the same bcToken by the same person.
    // in this moment, this candidate can see his own RateOtherRecord for realizing self rating point
}
