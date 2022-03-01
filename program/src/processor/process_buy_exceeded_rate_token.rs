//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Buy instruction
pub fn process_buy_exceeded_rate_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    token: Pubkey,
    amount: u64,
) -> ProgramResult {
    // Almost the same as methods that CandidateLimitRecord has done with by using OutsideBuyerRecord
    // we need to set time limits for this functions in some way.

    let account_info_iter = &mut accounts.iter();

    let outside_buyer_info = next_account_info(account_info_iter)?; // 0
    let outside_buyer_record_info = next_account_info(account_info_iter)?; // 1

    let seller_token_info = next_account_info(account_info_iter)?; // 2
    let buyer_token_source_info = next_account_info(account_info_iter)?; // 3

    let buyer_token_receive_info = next_account_info(account_info_iter)?; // 4

    let outside_buyer_holding_info = next_account_info(account_info_iter)?; // 5
    let seller_token_mint_info = next_account_info(account_info_iter)?; // 6

    let system_info = next_account_info(account_info_iter)?; // 7

    let clock_info = next_account_info(account_info_iter)?; // 8
    let clock = Clock::from_account_info(clock_info)?;

    let rent_sysvar_info = next_account_info(account_info_iter)?; // 9
    let rent = &Rent::from_account_info(rent_sysvar_info)?;

}
