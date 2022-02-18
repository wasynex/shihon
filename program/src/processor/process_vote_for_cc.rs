//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Vote instruction
pub fn process_vote_for_cc(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // we can choose fixed and fluid voting
    // and VoteSource has some parameters: pull and push
    // pull need less amount than your ring, push need more amount than your ring
}
