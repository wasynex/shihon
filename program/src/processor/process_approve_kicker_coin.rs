//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Approve instruction
pub fn process_approve_kicker_coin(program_id: &Pubkey, accounts: &[AccountInfo],coordinator_input: String,) -> ProgramResult {
    unimplemented!();
    // this function make coordinator to input some text message to first kicker
    // that text make rater and init content holder to create RFT for rating other
    // this minting can only when first kicker and coordinator stay in same tanistry
    // At this point, we need to create a zero version of CandidateLimitRecord to prepare for the candidate
    // The program to create the mpc is in this record

    let account_info_iter = &mut accounts.iter();

    let kicker_info = next_account_info(account_info_iter)?; // 0
    let coordinator_info = next_account_info(account_info_iter)?; // 1
    let tanistry_info = next_account_info(account_info_iter)?; // 2
    let coordinator_input_info = next_account_info(account_info_iter)?; // 3
    let kicker_token_source_info = next_account_info(account_info_iter)?; // 4
    let tanistry_authority_info = next_account_info(account_info_iter)?; // 5
    let coordinator_receive_token_info = next_account_info(account_info_iter)?; // 6
    let kicker_coin_owner_record_info = next_account_info(account_info_iter)?; // 7
    let kicker_receive_token_info = next_account_info(account_info_iter)?; // 8
    let candidate_token_holding_info = next_account_info(account_info_iter)?; // 8
    let system_info = next_account_info(account_info_iter)?; // 10
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 11
    let rent = &Rent::from_account_info(rent_sysvar_info)?;
    let clock_info = next_account_info(account_info_iter)?; // 12
    let clock = Clock::from_account_info(clock_info)?;

    assert_can_approve_kicker_coin();

    assert_valid_coordinator_input_on_kicker_coin();


}
