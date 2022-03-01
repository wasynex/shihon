//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Rate instruction
pub fn process_rate_other(program_id: &Pubkey, accounts: &[AccountInfo],rating: u64) -> ProgramResult {
    unimplemented!();
    // this func has three step
    // we need a multisig that combines two keys each for calling the one candidate or first kicker
    // of the three, only two are trials that raters can be consciously aware of
    // Step 1: Issue a MixContentRecord for mixing init content and buddy candidate's content
    // Step 2: after receive content's link from buddy candidate, put amount of rating point on buddy candidate
    // Step 3: that point need to turn the new minting RFT by first kicker and coordinator's input

    // --for the second rating--
    // the procedure is almost the same as the first rating
    // What's different is that system computes various INDEX on basis of CandidateLimitRecord and RateOtherRecord
    // If conditions are right, OutsideBuyerRecord would be issued for selling the exceeded rate token
    // And if the number of rounds exceeds the specified, CCVoteRecord also would be issued for voting for CC

    let account_info_iter = &mut accounts.iter();

    let rater_candidate_info = next_account_info(account_info_iter)?; // 0
    let rating_input_info = next_account_info(account_info_iter)?; // 1
    let buddy_candidate_info = next_account_info(account_info_iter)?; // 2
    let kicker_token_info = next_account_info(account_info_iter)?; // 3

    let coordinator_input_info = next_account_info(account_info_iter)?; // 4

    let tanistry_token_holding_info = next_account_info(account_info_iter)?; // 5
    let mix_content_record_info = next_account_info(account_info_iter)?; // 6

    let rating_info = next_account_info(account_info_iter)?; // 7
    let system_info = next_account_info(account_info_iter)?; // 8

    let rent_sysvar_info = next_account_info(account_info_iter)?; // 8
    let rent = &Rent::from_account_info(rent_sysvar_info)?;

}
