//! Mix and Rate Account

use borsh::maybestd::io::Write;
use std::cmp::Ordering;

use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::{Slot, UnixTimestamp};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{
    error::ShihonError,
    state::{
        enums::{
            InstructionExecutionFlags, InstructionExecutionStatus, MintMaxVoteWeightSource,
            ProposalState, ShihonAccountType, VoteThresholdPercentage,
        },
        governance::GovernanceConfig,
        proposal_instruction::ProposalInstructionV2,
        realm::Realm,
        vote_record::Vote,
    },
    PROGRAM_AUTHORITY_SEED,
};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};


#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum Mix {
    /// Mix on the option is not resolved yet
    pub link: Pubkey,

    /// Mix on the option is completed and the option passed
    pub buddy_candidate: Pubkey,

    /// Mix on the option is completed and the option was defeated
    pub rater_candidate: Pubkey,

    /// Mix content result state before rating action
    pub option_mix_result: OptionMixResult,
}

/// Mix content result state before rating action
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum OptionMixResult {
    /// Mix on the option is not resolved yet
    None,

    /// Mix on the option is completed and the option passed
    Succeeded,

    /// Mix on the option is completed and the option was defeated
    Defeated,
}

/// Rate Option
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RateOption {
    /// Option label
    pub label: String,

    /// Rate weight for the option
    pub rate_weight: u64,

    /// Mix result for the option
    pub mix_result: OptionMixResult,

    /// The number of rating action already executed
    pub rating_action_executed_count: u16,

    /// The index of the the next instruction to be added
    pub instructions_next_index: u16,
}

/// Rating vote type
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum RateType {
    /// Single choice vote with mutually exclusive choices
    /// In the SingeChoice mode there can ever be a single winner
    /// If multiple options score the same highest vote then the Proposal is not resolved and considered as Failed
    /// Note: Yes/No vote is a single choice (Yes) vote with the deny option (No)
    SingleChoice,

    /// Multiple options can be selected with up to N choices per voter
    /// By default N equals to the number of available options
    /// Note: In the current version the N limit is not supported and not enforced yet
    MultiChoice(u16),
}

/// state for rating the content
pub struct Rate {
    pub is_initialized: bool,
    pub rated_content: Pubkey,
    pub rater_pubkey: Pubkey,
}

///TODO: need to make mix modules with rating function
impl IsInitialized for Rate {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
/// Deserializes Proposal and validates it belongs to the given Governance and Governing Mint
pub fn get_proposal_data_for_governance_and_governing_mint(
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

/// Deserializes Proposal and validates it belongs to the given Governance
pub fn get_proposal_data_for_governance(
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

/// Returns Proposal PDA seeds
pub fn get_proposal_address_seeds<'a>(
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

/// Returns Proposal PDA address
pub fn get_proposal_address<'a>(
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

/// Assert options to create proposal are valid for the Proposal vote_type
pub fn assert_valid_proposal_options(
    options: &[String],
    vote_type: &RateType,
) -> Result<(), ProgramError> {
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
