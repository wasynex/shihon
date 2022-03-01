//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes DiscardBcToken instruction
pub fn process_discard_bc_token(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
   /// 0. `[writable]` bcToken account. PDA seeds:['bcToken', config]
    /// 1. `[signer]` bcToken authority
    /// 2. `[]` bcToken Mint
    /// 3. `[signer]` Payer
    let account_info_iter = &mut accounts.iter();

    let bc_token_info = next_account_info(account_info_iter)?; // 0
    let bc_token_authority_info = next_account_info(account_info_iter)?; // 1
    let bc_token_mint_info = next_account_info(account_info_iter)?; // 2
    let payer_info = next_account_info(account_info_iter)?; // 3


    assert_can_discard_bc_token()?;


}
