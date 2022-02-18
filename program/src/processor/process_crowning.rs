//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Crowning instruction
pub fn process_crowning(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // Update the ownership of KickerCoin: from first kicker to crown
    // The crown is elected from candidates in the same tanistry
    // Note: Every time the ownership of KickerCoin leaves the tanistry, the building hash must be kept updated.

}
