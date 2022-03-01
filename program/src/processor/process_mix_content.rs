//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes mix content instruction
pub fn process_mix_content(program_id: &Pubkey, accounts: &[AccountInfo],time_shift_a: u64,
        time_shift_b: u64) -> ProgramResult {

    // If you do not agree to mix the content, you lose the right to rate on next process
    // The elements needed to mix are as follows.
    // 1. input of coordinator
    // 2. Init content info on Metadata in First Kicker's bcToken
    // 3. Buddy content info on Metadata in buddy Candidate's bcToken

    let account_info_iter = &mut accounts.iter();

    let coordinator_info = next_account_info(account_info_iter)?; // 0
    let rater_candidate_info = next_account_info(account_info_iter)?; // 1
    let buddy_candidate_info = next_account_info(account_info_iter)?; // 2
    let kicker_coin_owner_record_info = next_account_info(account_info_iter)?; // 3
    let coordinator_input_info = next_account_info(account_info_iter)?; // 4
    let tanistry_token_holding_info = next_account_info(account_info_iter)?; // 5
    let content_authority_info = next_account_info(account_info_iter)?; // 6
    let buddy_candidate_info = next_account_info(account_info_iter)?; // 7
    let mix_content_record_info = next_account_info(account_info_iter)?; // 8
    let rating_info = next_account_info(account_info_iter)?; // 9
    let system_info = next_account_info(account_info_iter)?; // 10
    let rent = &Rent::from_account_info(rent_sysvar_info)?; // 11



}
