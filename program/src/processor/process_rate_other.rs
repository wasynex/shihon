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
pub fn process_rate_other(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // this func has three step
    // we need a multisig that combines two keys each for calling the one candidate or first kicker
    // of the three, only two are trials that raters can be consciously aware of
    // Step 1: Issue a MixContentRecord for mixing init content and buddy candidate's content
    // Step 2: after receive content's link from buddy candidate, put amount of rating point on buddy candidate
    // Step 3: that point need to turn the new minting RFT by first kicker and coordinator's input

    // In the second rating
    // the procedure is almost the same as the first rating
    // What's different is that system computes various INDEX on basis of CandidateLimitRecord and RateOtherRecord
    // If conditions are right, OutsideBuyerRecord would be issued for selling the exceeded rate token
    // And if the number of rounds exceeds the specified, CCVoteRecord also would be issued for voting for CC
}
