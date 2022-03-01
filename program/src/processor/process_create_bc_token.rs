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
pub fn process_create_bc_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    amount: u64,
    config: BcTokenMetadata,
) -> ProgramResult {
    // This function's role proceeds in two different stages
    // Step 1: creating bcToken as DraftBlankCheck only has a creator's Pubkey, means with no information of content itself
    // Step 2: post a bcToken as DraftBlankCheck into Oracle, and then receive some metadata from the Oracle
    // metadata has content's fingerprint certify that it has been stored securely and some other information about each content
    // After these two processes, bcToken is completed for using as first kicking
    // Note: At this moment, we would not write code about generating a metadata
    // Note: any payment information exists in KickerCoinOwnerRecord or CandidateLimitRecord, not in bcToken itself
    let account_info_iter = &mut accounts.iter();

    let bc_token_info = next_account_info(account_info_iter)?; // 0
    let bc_token_authority_info = next_account_info(account_info_iter)?; // 1
    let bc_token_mint_info = next_account_info(account_info_iter)?; // 2
    let kicker_coin_token_holding_info = next_account_info(account_info_iter)?; // 3
    let payer_info = next_account_info(account_info_iter)?; // 4
    let candidate_token_holding_info = next_account_info(account_info_iter)?; // 5
    let governance_authority_info = next_account_info(account_info_iter)?; // 6
    let system_info = next_account_info(account_info_iter)?; // 7
    let spl_token_info = next_account_info(account_info_iter)?; // 8
    let rent_sysvar_info = next_account_info(account_info_iter)?; // 9
    let rent = &Rent::from_account_info(rent_sysvar_info)?;


    assert_create_bc_token(program_id, &config, bc_token_info);


    create_and_serialize_account_signed::<BcTokenMetadata>(
        payer_info,
        bc_token_info,
        &get_proposal_instruction_address_seeds(
            proposal_info.key,
            &option_index.to_le_bytes(),
            &instruction_index.to_le_bytes(),
        ),
        program_id,
        system_info,
        rent,
    )?;

    Ok(())
}
