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
    // This function's role proceeds in two different stages
    // Step 1: creating bcToken as DraftBlankCheck only has a creator's Pubkey, means with no information of content itself
    // Step 2: post a bcToken as DraftBlankCheck into Oracle, and then receive some metadata from the Oracle
    // metadata has content's fingerprint certify that it has been stored securely and some other information about each content
    // After these two processes, bcToken is completed for using as first kicking
    // Note: At this moment, we would not write code about generating a metadata
    // Note: any payment information exists in KickerCoinOwnerRecord or CandidateLimitRecord, not in bcToken itself
}
