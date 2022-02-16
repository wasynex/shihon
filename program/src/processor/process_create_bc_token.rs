//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes CreateBcToken instruction
pub fn process_create_bc_token(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
}
