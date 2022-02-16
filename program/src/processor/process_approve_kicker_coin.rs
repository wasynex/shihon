//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

pub fn approve_kicker_coin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
}
