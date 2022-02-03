pub mod process_approve_kicker_coin;
pub mod process_bump_self_rate;
pub mod process_candidate;
pub mod process_create_bc_token;
pub mod process_create_rate_other_record;
pub mod process_create_token_buyer_record;
pub mod process_crowning;
pub mod process_kick_to_coordinator;
pub mod process_rate_other;
pub mod process_sellout;
pub mod process_vote_for_cc;

use process_approve_kicker_coin::*;
use process_bump_self_rate::*;
use process_candidate::*;
use process_create_bc_token::*;
use process_create_rate_other_record::*;
use process_create_token_buyer_record::*;
use process_crowning::*;
use process_kick_to_coordinator::*;
use process_rate_other::*;
use process_sellout::*;
use process_vote_for_cc::*;

use crate::instruction::ShihonInstruction;

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

/// Processes an instruction
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    unimplemented!();
}

fn is_go_forward_to_process() {
    unimplemented!();
}

fn count_percentage_done_process_member() {
    unimplemented!();
}
