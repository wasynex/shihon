//! Program processor

mod process_approve_kicker_coin;
mod process_bump_self_rate;
mod process_buy_exceeded_rate_token;
mod process_candidate;
mod process_create_bc_token;
mod process_crowning;
mod process_deny_kicker_coin;
mod process_discard_bc_token;
mod process_draft_blank_check;
mod process_kick_to_coordinator;
mod process_mix_content;
mod process_rate_other;
mod process_vote_for_cc;

use crate::instruction::ShihonInstruction;

use process_approve_kicker_coin::*;
use process_bump_self_rate::*;
use process_buy_exceeded_rate_token::*;
use process_candidate::*;
use process_create_bc_token::*;
use process_crowning::*;
use process_deny_kicker_coin::*;
use process_discard_bc_token::*;
use process_draft_blank_check::*;
use process_kick_to_coordinator::*;
use process_mix_content::*;
use process_rate_other::*;
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

    ShihonInstruction::DraftBlankCheck { name } => process_draft_blank_check(program_id, accounts),

    ShihonInstruction::CreateBcToken {
        amount,
        config
    } => process_create_bc_token(program_id, accounts),

    ShihonInstruction::DiscardBcToken {} => process_discard_bc_token(program_id, accounts),

    ShihonInstruction::KickingToCoordinator { coordinator, amount } => process_kick_to_coordinator(program_id, accounts),

    ShihonInstruction::ApproveKickerCoin {
        coordinator_input
    } => process_approve_kicker_coin(program_id, accounts),

    ShihonInstruction::DenyKickerCoin {} => process_deny_kicker_coin(program_id, accounts),

    ShihonInstruction::Candidate { coordinator, amount }, {} => process_candidate(program_id, accounts),

    ShihonInstruction::MixContent {
        time_shift_a,
        time_shift_b,
    } => process_mix_content(program_id, accounts),

    ShihonInstruction::RateOtherContent { rating } => process_rate_other(program_id, accounts),

    ShihonInstruction::BumpSelfRate { amount } => process_bump_self_rate(program_id, accounts),

    ShihonInstruction::BuyExceededRateToken { token, amount } => process_buy_exceeded_rate_token(program_id, accounts),

    ShihonInstruction::Crowning { crown } => process_crowning(program_id, accounts),

    ShihonInstruction::VoteForCC {
        target_ring
    } => process_vote_for_cc(program_id, accounts),

}

/// Checks whether bcToken account can go forward to the next process
fn assert_can_go_forward_to_the_process() {
    unimplemented!();
}
/// Checks that how many percentage of process has done by all the member
fn count_percentage_done_process_member() {
    unimplemented!();
}
