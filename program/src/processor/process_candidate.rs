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
pub fn process_candidate(program_id: &Pubkey, accounts: &[AccountInfo],coordinator: Pubkey, amount: u64) -> ProgramResult {
    // The previously created CandidateLimitRecord has a mechanism to limit the number of coins and people called candidate limit bar
    // CandidateLimitRecord is issued for each new candidate, always referring to the previous record to calculate the number of people and the total coin limit
    // Note: The tanistry will not be formed until the last candidate arrives
    // Note: If you want to know what happens next...Look at state/tanistry.rs

    let account_info_iter = &mut accounts.iter();

    let candidate_info = next_account_info(account_info_iter)?; // 0
    let candidate_source_token_info = next_account_info(account_info_iter)?; // 1
    let tanistry_token_holding_info = next_account_info(account_info_iter)?; // 2
    let candidate_receive_token_info = next_account_info(account_info_iter)?; // 3
    let kicker_receive_token_info = next_account_info(account_info_iter)?; // 4
    let candidate_limit_record_info = next_account_info(account_info_iter)?; // 5
    let tanistry_token_holding_info = next_account_info(account_info_iter)?; // 6
    let spl_token_info = next_account_info(account_info_iter)?; // 7
    let system_info = next_account_info(account_info_iter)?; // 8
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 9
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
}
