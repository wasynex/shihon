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
pub fn process_kick_to_coordinator(program_id: &Pubkey, accounts: &[AccountInfo],coordinator: Pubkey, amount: u64) -> ProgramResult {
    // this function is for first kicker or crown
    // We need to make sure that targeted bcToken's state in PublicOtherGround or HoldingOnPrivate
    // Now, issue new KickerCoinOwnerRecord. This Record would have Building hash and some Variables for example, round: u8, amount_of_kicker_coin:u64
    // It will be issued the KickerCoinOwnerRecord even if the coordinator does not approve it because this record's role must map each crown and keep your KickerCoin safe.
    // Note: The coordinator will be notified. I don't know how exactly to do that.
    // Need to create validation that the content is compliant. I consider that after create bcToken metadata

    ///   2. `[signer]` The account of the person as first kicker initializing the two BcToken into Tanistry Ring
    ///   3. `[writable]` KickerCoin Owner Record PDA seeds: ['KickerCoinOwnerRecord', coordinator ]
    ///   4. `[writable]` Coordinator's bcToken account to flag as received KickerCoin from first kicker
    ///   5. `[]` Clock sysvar
    let account_info_iter = &mut accounts.iter();

    let kicker_info = next_account_info(account_info_iter)?; // 0
    let token_owner_record_info = next_account_info(account_info_iter)?; // 1
    let kicker_coin_info = next_account_info(account_info_iter)?; // 2
    let kicker_coin_owner_record_info = next_account_info(account_info_iter)?; // 3
    let payer_info = next_account_info(account_info_iter)?; // 4
    let clock_info = next_account_info(account_info_iter)?; //5
    let clock = Clock::from_account_info(clock_info)?;
}
