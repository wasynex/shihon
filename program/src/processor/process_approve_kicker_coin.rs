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
pub fn process_approve_kicker_coin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // this function make coordinator to input some text message to first kicker
    // that text make rater and init content holder to create RFT for rating other
    // this minting can only when first kicker and coordinator stay in same tanistry
    // At this point, we need to create a zero version of CandidateLimitRecord to prepare for the candidate
    // The program to create the mpc is in this record
}
