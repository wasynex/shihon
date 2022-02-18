//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_governance_tools::account::create_and_serialize_account_signed;

/// Processes Kicking instruction
pub fn process_kick_to_coordinator(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    unimplemented!();
    // this function is for first kicker or crown
    // We need to make sure that targeted bcToken's state in PublicOtherGround or HoldingOnPrivate
    // Now, issue new KickerCoinOwnerRecord. This Record would have Building hash and some Variables for example, round: u8, amount_of_kicker_coin:u64
    // It will be issued the KickerCoinOwnerRecord even if the coordinator does not approve it because this record's role must map each crown and keep your KickerCoin safe.
    // Note: The coordinator will be notified. I don't know how exactly to do that.
    // Need to create validation that the content is compliant. I consider that after create bcToken metadata
}
