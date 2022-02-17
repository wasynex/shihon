//! Program processor

mod process_approve_kicker_coin;
mod process_bump_self_rate;
mod process_buy_exceeded_rate_token;
mod process_candidate;
mod process_create_bc_token;
// mod process_create_candidate_limit_record;
// mod process_create_cc_vote_record;
// mod process_create_outside_buyer_record;
// mod process_create_rate_other_record;
mod process_crowning;
mod process_kick_to_coordinator;
mod process_rate_other;
// mod process_sell_exceeded_rate_token;
mod process_deny_kicker_coin;
mod process_vote_for_cc;

use crate::instruction::ShihonInstruction;

use process_approve_kicker_coin::*;
use process_bump_self_rate::*;
use process_buy_exceeded_rate_token::*;
use process_candidate::*;
use process_create_bc_token::*;
// use process_create_candidate_limit_record::*;
// use process_create_cc_vote_record::*;
// use process_create_outside_buyer_record::*;
// use process_create_rate_other_record::*;
use process_crowning::*;
use process_kick_to_coordinator::*;
use process_rate_other::*;
// use process_sell_exceeded_rate_token::*;
use process_deny_kicker_coin::*;
use process_vote_for_cc::*;

use solana_program::{
    account_info::AccountInfo, borsh::try_from_slice_unchecked, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
    // Use try_from_slice_unchecked to support forward compatibility of newer UI with older program
    let instruction: ShihonInstruction =
        try_from_slice_unchecked(input).map_err(|_| ProgramError::InvalidInstructionData)?;

    unimplemented!();
}

/// Checks whether bcToken account can go forward to the next process
fn assert_can_go_forward_to_the_process() {
    unimplemented!();
}
/// Checks that how many percentage of process has done by all the member
fn count_percentage_done_process_member() {
    unimplemented!();
}
