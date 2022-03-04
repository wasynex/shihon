//! MixContentRecord Account

use borsh::maybestd::io::Write;
use std::cmp::Ordering;

use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::{Slot, UnixTimestamp};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{error::ShihonError, state::enums::ShihonAccountType, PROGRAM_AUTHORITY_SEED};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct MixContentRecord {
    ///
    pub account_type: ShihonAccountType,

    /// Encrypted link
    pub encrypted_link: String,

    /// Rater candidate (It's you!)
    pub rater_candidate: Pubkey,

    /// Buddy candidate
    pub buddy_candidate: Pubkey,

    /// Mix content result state before rating action
    pub option_mix_result: OptionMixResult,

    /// Finger print of Mixed Content
    pub finger_print_of_mixed_content: String,
}

/// Mix content result state before rating action
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum OptionMixResult {
    /// Mix on the option is not resolved yet
    None,

    /// Mix on the option is completed and the option passed
    Succeeded,

    /// Mix on the option was defeated
    Defeated,
}

impl AccountMaxSize for MixContentRecord {}

impl IsInitialized for MixContentRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::MixContentRecord
    }
}

impl MixContentRecord {
    pub fn assert_finished_shaffle_on_candidate_list() {
        unimplemented!();
    }

    pub fn assert_is_valid_triangle_person() {
        unimplemented!();
    }

    pub fn assert_can_mix_the_content() {
        unimplemented!();
    }

    /// mixing the content for first step
    fn multi_sig_1() {
        unimplemented!();
    }
}

/// Deserializes Proposal and validates it belongs to the given content
pub fn get_content_metadata_for_mixing(
    program_id: &Pubkey,
    proposal_info: &AccountInfo,
    governance: &Pubkey,
    governing_token_mint: &Pubkey,
) -> Result<Rate, ProgramError> {
    let proposal_data = get_proposal_data_for_governance(program_id, proposal_info, governance)?;

    if proposal_data.governing_token_mint != *governing_token_mint {
        return Err(ShihonError::InvalidGoverningMintForProposal.into());
    }

    Ok(proposal_data)
}

/// Deserializes rate option and validates it belongs to the given two content
pub fn get_content_metadata_for_rating(
    program_id: &Pubkey,
    proposal_info: &AccountInfo,
    governance: &Pubkey,
) -> Result<Rate, ProgramError> {
    let proposal_data = get_proposal_data(program_id, proposal_info)?;

    if proposal_data.governance != *governance {
        return Err(ShihonError::InvalidGovernanceForProposal.into());
    }

    Ok(proposal_data)
}

/// Returns Rate Option PDA seeds
pub fn get_rate_option_address_seeds<'a>(
    governance: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
    proposal_index_le_bytes: &'a [u8],
) -> [&'a [u8]; 4] {
    [
        PROGRAM_AUTHORITY_SEED,
        governance.as_ref(),
        governing_token_mint.as_ref(),
        proposal_index_le_bytes,
    ]
}

/// Returns rate option PDA address
pub fn get_rate_option_address<'a>(
    program_id: &Pubkey,
    governance: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
    proposal_index_le_bytes: &'a [u8],
) -> Pubkey {
    Pubkey::find_program_address(
        &get_proposal_address_seeds(governance, governing_token_mint, proposal_index_le_bytes),
        program_id,
    )
    .0
}

/// Assert can create rate options for holding some key
pub fn assert_valid_rate_options(options: &[String], vote_type: &R) -> Result<(), ProgramError> {
    if options.is_empty() {
        return Err(ShihonError::InvalidProposalOptions.into());
    }

    if let RateType::MultiChoice(n) = *vote_type {
        if options.len() == 1 || n as usize != options.len() {
            return Err(ShihonError::InvalidProposalOptions.into());
        }
    }

    if options.iter().any(|o| o.is_empty()) {
        return Err(ShihonError::InvalidProposalOptions.into());
    }

    Ok(())
}
