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
pub fn process_mix_content(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // If you do not agree to mix the content, you lose the right to rate on next process
    // The elements needed to mix are as follows.
    // 1. input of coordinator
    // 2. Init content info on Metadata in First Kicker's bcToken
    // 3. Buddy content info on Metadata in buddy Candidate's bcToken
}
