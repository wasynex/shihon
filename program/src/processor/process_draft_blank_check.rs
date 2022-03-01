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
pub fn process_draft_blank_check(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    // this processor can create blank check with no content information
    // blank check holds only issuer pubkey and how much deposit for its content
    // when bump own content self rate, you can input only this blank check
    let bc_info = next_account_info(account_info_iter)?; // 0
    let bc_authority_info = next_account_info(account_info_iter)?; // 1
    let system_info = next_account_info(account_info_iter)?; // 2
    let token_info = next_account_info(account_info_iter)?; // 3
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 4

    assert_valid_create_blank_check(program_id, &name, bc_info)?;

    let bc_data_info = get_be_data(program_id, bc_info);

    create_and_serialize_account_signed::<Governance>(
        payer_info,
        bc_data_info,
        &get_program_governance_address_seeds(bc_data_info.key),
        program_id,
        system_info,
        rent,
    )?;

    Ok(())
}
