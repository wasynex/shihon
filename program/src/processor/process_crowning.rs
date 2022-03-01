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
pub fn process_crowning(program_id: &Pubkey, accounts: &[AccountInfo],crown: Pubkey) -> ProgramResult {
    // Update the ownership of KickerCoin: from first kicker to crown
    // The crown is elected from candidates in the same tanistry
    // Note: Every time the ownership of KickerCoin leaves the tanistry, the building hash must be kept updated.

    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Coordinator's token account for send his input
    /// 2. `[writable]` The Coordinator's token account for receive the NFT as Refund when he drop the game
    /// 3. `[writable]` KickerCoin Owner Record(from Coordinator to Candidate as next Crown)
    /// 4. `[writable]` The new Crown's token account for choosing next coordinator
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
    /// 6. `[]` The token program
    /// 7. `[]` System program
    /// 8. `[]` Sysvar Rent

    let account_info_iter = &mut accounts.iter();

    let realm_info = next_account_info(account_info_iter)?; // 0
    let program_governance_info = next_account_info(account_info_iter)?; // 1

    let governed_program_info = next_account_info(account_info_iter)?; // 2
    let governed_program_data_info = next_account_info(account_info_iter)?; // 3
    let governed_program_upgrade_authority_info = next_account_info(account_info_iter)?; // 4

    let token_owner_record_info = next_account_info(account_info_iter)?; // 5

    let payer_info = next_account_info(account_info_iter)?; // 6
    let bpf_upgrade_loader_info = next_account_info(account_info_iter)?; // 7

    let system_info = next_account_info(account_info_iter)?; // 8

    let rent_sysvar_info = next_account_info(account_info_iter)?; // 9
    let rent = &Rent::from_account_info(rent_sysvar_info)?;

}
