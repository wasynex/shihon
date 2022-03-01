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
pub fn process_bump_self_rate(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    // what this func needed is only some validators
    // for update amount of self rating as coin on CandidateLimitRecord, the candidate need to issue same bcToken again without metadata. if want stay his rate same, bcToken with 0 deposit be permitted to issue. and we need to verify that it is the same bcToken by the same person.
    // in this moment, this candidate can see his own RateOtherRecord for realizing self rating point

    let account_info_iter = &mut accounts.iter();

    let bc_token_info = next_account_info(account_info_iter)?; // 0
    let bc_token_authority_info = next_account_info(account_info_iter)?; // 1

    let candidate_token_source_info = next_account_info(account_info_iter)?; // 2
    let candidate_token_info = next_account_info(account_info_iter)?; // 3

    let tanistry_token_holding_info = next_account_info(account_info_iter)?; // 4

    let candidate_limit_record_info = next_account_info(account_info_iter)?; // 5
    let spl_token_info = next_account_info(account_info_iter)?; // 6

    let system_info = next_account_info(account_info_iter)?; // 7

    let rent_sysvar_info = next_account_info(account_info_iter)?; // 8
    let rent = &Rent::from_account_info(rent_sysvar_info)?;

}
