//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes draft blank check instruction
pub fn process_draft_blank_check(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // this processor can create blank check with no content information
    // blank check holds only issuer pubkey and how much deposit for its content
    // when bump own content self rate, you can input only this blank check
}
