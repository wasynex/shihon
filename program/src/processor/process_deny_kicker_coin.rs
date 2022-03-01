//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Deny instruction
pub fn process_deny_kicker_coin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();

    let kicker_info = next_account_info(account_info_iter)?; // 0
    let tanistry_authority_info = next_account_info(account_info_iter)?; // 1
    let kicker_coin_owner_record_info = next_account_info(account_info_iter)?; // 2
    let kicker__receive_token_info = next_account_info(account_info_iter)?; // 3
    let candidate_token_holding_info = next_account_info(account_info_iter)?; // 4

}
