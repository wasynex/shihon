use crate::PROGRAM_AUTHORITY_SEED;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::clock::{Slot, UnixTimestamp};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// state for rating the content
pub struct Rate {
    pub is_initialized: bool,
    pub rated_content: Pubkey,
    pub rater_pubkey: Pubkey,
    pub rate_amount: u64,
    pub temp_rating_account_pubkey: Pubkey,
}

impl Sealed for Rate {}

impl IsInitialized for Rate {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Rate {
    const LEN: usize = 96;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        unimplemented!();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        unimplemented!();
    }
}



//! Proposal  Account

use borsh::maybestd::io::Write;
use std::cmp::Ordering;

use solana_program::borsh::try_from_slice_unchecked;
use solana_program::clock::{Slot, UnixTimestamp};

use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::state::legacy::ProposalV1;
use crate::{
    error::ShihonError,
    state::{
        enums::{
            ShihonAccountType, InstructionExecutionFlags, InstructionExecutionStatus,
            MintMaxVoteWeightSource, ProposalState, VoteThresholdPercentage,
        },
        governance::GovernanceConfig,
        proposal_instruction::ProposalInstructionV2,
        realm::Realm,
        vote_record::Vote,
    },
    PROGRAM_AUTHORITY_SEED,
};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Proposal option vote result
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum OptionVoteResult {
    /// Vote on the option is not resolved yet
    None,

    /// Vote on the option is completed and the option passed
    Succeeded,

    /// Vote on the option is completed and the option was defeated
    Defeated,
}

/// Proposal Option
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ProposalOption {
    /// Option label
    pub label: String,

    /// Vote weight for the option
    pub vote_weight: u64,

    /// Vote result for the option
    pub vote_result: OptionVoteResult,

    /// The number of the instructions already executed
    pub instructions_executed_count: u16,

    /// The number of instructions included in the option
    pub instructions_count: u16,

    /// The index of the the next instruction to be added
    pub instructions_next_index: u16,
}

/// Proposal vote type
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum VoteType {
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

/// Governance Proposal
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct ProposalV2 {
    /// Governance account type
    pub account_type: ShihonAccountType,

    /// Governance account the Proposal belongs to
    pub governance: Pubkey,

    /// Indicates which Governing Token is used to vote on the Proposal
    /// Whether the general Community token owners or the Council tokens owners vote on this Proposal
    pub governing_token_mint: Pubkey,

    /// Current proposal state
    pub state: ProposalState,

    // TODO: add state_at timestamp to have single field to filter recent proposals in the UI
    /// The TokenOwnerRecord representing the user who created and owns this Proposal
    pub token_owner_record: Pubkey,

    /// The number of signatories assigned to the Proposal
    pub signatories_count: u8,

    /// The number of signatories who already signed
    pub signatories_signed_off_count: u8,

    /// Vote type
    pub vote_type: VoteType,

    /// Proposal options
    pub options: Vec<ProposalOption>,

    /// The weight of the Proposal rejection votes
    /// If the proposal has no deny option then the weight is None
    /// Only proposals with the deny option can have executable instructions attached to them
    /// Without the deny option a proposal is only non executable survey
    pub deny_vote_weight: Option<u64>,

    /// When the Proposal was created and entered Draft state
    pub draft_at: UnixTimestamp,

    /// When Signatories started signing off the Proposal
    pub signing_off_at: Option<UnixTimestamp>,

    /// When the Proposal began voting as UnixTimestamp
    pub voting_at: Option<UnixTimestamp>,

    /// When the Proposal began voting as Slot
    /// Note: The slot is not currently used but the exact slot is going to be required to support snapshot based vote weights
    pub voting_at_slot: Option<Slot>,

    /// When the Proposal ended voting and entered either Succeeded or Defeated
    pub voting_completed_at: Option<UnixTimestamp>,

    /// When the Proposal entered Executing state
    pub executing_at: Option<UnixTimestamp>,

    /// When the Proposal entered final state Completed or Cancelled and was closed
    pub closed_at: Option<UnixTimestamp>,

    /// Instruction execution flag for ordered and transactional instructions
    /// Note: This field is not used in the current version
    pub execution_flags: InstructionExecutionFlags,

    /// The max vote weight for the Governing Token mint at the time Proposal was decided
    /// It's used to show correct vote results for historical proposals in cases when the mint supply or max weight source changed
    /// after vote was completed.
    pub max_vote_weight: Option<u64>,

    /// The vote threshold percentage at the time Proposal was decided
    /// It's used to show correct vote results for historical proposals in cases when the threshold
    /// was changed for governance config after vote was completed.
    pub vote_threshold_percentage: Option<VoteThresholdPercentage>,

    /// Proposal name
    pub name: String,

    /// Link to proposal's description
    pub description_link: String,
}

impl AccountMaxSize for ProposalV2 {
    fn get_max_size(&self) -> Option<usize> {
        let options_size: usize = self.options.iter().map(|o| o.label.len() + 19).sum();
        Some(self.name.len() + self.description_link.len() + options_size + 199)
    }
}

impl IsInitialized for ProposalV2 {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::ProposalV2
    }
}

impl ProposalV2 {
    /// Checks if Signatories can be edited (added or removed) for the Proposal in the given state
    pub fn assert_can_edit_signatories(&self) -> Result<(), ProgramError> {
        self.assert_is_draft_state()
            .map_err(|_| ShihonError::InvalidStateCannotEditSignatories.into())
    }

    /// Checks if Proposal can be singed off
    pub fn assert_can_sign_off(&self) -> Result<(), ProgramError> {
        match self.state {
            ProposalState::Draft | ProposalState::SigningOff => Ok(()),
            ProposalState::Executing
            | ProposalState::ExecutingWithErrors
            | ProposalState::Completed
            | ProposalState::Cancelled
            | ProposalState::Voting
            | ProposalState::Succeeded
            | ProposalState::Defeated => Err(ShihonError::InvalidStateCannotSignOff.into()),
        }
    }

    /// Checks the Proposal is in Voting state
    fn assert_is_voting_state(&self) -> Result<(), ProgramError> {
        if self.state != ProposalState::Voting {
            return Err(ShihonError::InvalidProposalState.into());
        }

        Ok(())
    }

    /// Checks the Proposal is in Draft state
    fn assert_is_draft_state(&self) -> Result<(), ProgramError> {
        if self.state != ProposalState::Draft {
            return Err(ShihonError::InvalidProposalState.into());
        }

        Ok(())
    }

    /// Checks if Proposal can be voted on
    pub fn assert_can_cast_vote(
        &self,
        config: &GovernanceConfig,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        self.assert_is_voting_state()
            .map_err(|_| ShihonError::InvalidStateCannotVote)?;

        // Check if we are still within the configured max_voting_time period
        if self.has_vote_time_ended(config, current_unix_timestamp) {
            return Err(ShihonError::ProposalVotingTimeExpired.into());
        }

        Ok(())
    }

    /// Checks whether the voting time has ended for the proposal
    pub fn has_vote_time_ended(
        &self,
        config: &GovernanceConfig,
        current_unix_timestamp: UnixTimestamp,
    ) -> bool {
        // Check if we passed vote_end_time determined by the configured max_voting_time period
        self.voting_at
            .unwrap()
            .checked_add(config.max_voting_time as i64)
            .unwrap()
            < current_unix_timestamp
    }

    /// Checks if Proposal can be finalized
    pub fn assert_can_finalize_vote(
        &self,
        config: &GovernanceConfig,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        self.assert_is_voting_state()
            .map_err(|_| ShihonError::InvalidStateCannotFinalize)?;

        // We can only finalize the vote after the configured max_voting_time has expired and vote time ended
        if !self.has_vote_time_ended(config, current_unix_timestamp) {
            return Err(ShihonError::CannotFinalizeVotingInProgress.into());
        }

        Ok(())
    }

    /// Finalizes vote by moving it to final state Succeeded or Defeated if max_voting_time has passed
    /// If Proposal is still within max_voting_time period then error is returned
    pub fn finalize_vote(
        &mut self,
        governing_token_mint_supply: u64,
        config: &GovernanceConfig,
        realm_data: &Realm,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        self.assert_can_finalize_vote(config, current_unix_timestamp)?;

        let max_vote_weight = self.get_max_vote_weight(realm_data, governing_token_mint_supply)?;

        self.state = self.resolve_final_vote_state(max_vote_weight, config)?;
        // TODO: set voting_completed_at based on the time when the voting ended and not when we finalized the proposal
        self.voting_completed_at = Some(current_unix_timestamp);

        // Capture vote params to correctly display historical results
        self.max_vote_weight = Some(max_vote_weight);
        self.vote_threshold_percentage = Some(config.vote_threshold_percentage.clone());

        Ok(())
    }

    /// Resolves final proposal state after vote ends
    /// It inspects all proposals options and resolves their final vote results
    fn resolve_final_vote_state(
        &mut self,
        max_vote_weight: u64,
        config: &GovernanceConfig,
    ) -> Result<ProposalState, ProgramError> {
        // Get the min vote weight required for options to pass
        let min_vote_threshold_weight =
            get_min_vote_threshold_weight(&config.vote_threshold_percentage, max_vote_weight)
                .unwrap();

        // If the proposal has a reject option then any other option must beat it regardless of the configured min_vote_threshold_weight
        let deny_vote_weight = self.deny_vote_weight.unwrap_or(0);

        let mut best_succeeded_option_weight = 0;
        let mut best_succeeded_option_count = 0u16;

        for option in self.options.iter_mut() {
            // Any positive vote (Yes) must be equal or above the required min_vote_threshold_weight and higher than the reject option vote (No)
            // The same number of positive (Yes) and rejecting (No) votes is a tie and resolved as Defeated
            // In other words  +1 vote as a tie breaker is required to succeed for the positive option vote
            if option.vote_weight >= min_vote_threshold_weight
                && option.vote_weight > deny_vote_weight
            {
                option.vote_result = OptionVoteResult::Succeeded;

                match option.vote_weight.cmp(&best_succeeded_option_weight) {
                    Ordering::Greater => {
                        best_succeeded_option_weight = option.vote_weight;
                        best_succeeded_option_count = 1;
                    }
                    Ordering::Equal => {
                        best_succeeded_option_count =
                            best_succeeded_option_count.checked_add(1).unwrap()
                    }
                    Ordering::Less => {}
                }
            } else {
                option.vote_result = OptionVoteResult::Defeated;
            }
        }

        let mut final_state = if best_succeeded_option_count == 0 {
            // If none of the individual options succeeded then the proposal as a whole is defeated
            ProposalState::Defeated
        } else {
            match self.vote_type {
                VoteType::SingleChoice => {
                    let proposal_state = if best_succeeded_option_count > 1 {
                        // If there is more than one winning option then the single choice proposal is considered as defeated
                        best_succeeded_option_weight = u64::MAX; // no winning option
                        ProposalState::Defeated
                    } else {
                        ProposalState::Succeeded
                    };

                    // Coerce options vote results based on the winning score (best_succeeded_vote_weight)
                    for option in self.options.iter_mut() {
                        option.vote_result = if option.vote_weight == best_succeeded_option_weight {
                            OptionVoteResult::Succeeded
                        } else {
                            OptionVoteResult::Defeated
                        };
                    }

                    proposal_state
                }
                VoteType::MultiChoice(_n) => {
                    // If any option succeeded for multi choice then the proposal as a whole succeeded as well
                    ProposalState::Succeeded
                }
            }
        };

        // None executable proposal is just a survey and is considered Completed once the vote ends and no more actions are available
        // There is no overall Success or Failure status for the Proposal however individual options still have their own status
        if self.deny_vote_weight.is_none() {
            final_state = ProposalState::Completed;
        }

        Ok(final_state)
    }

    /// Calculates max vote weight for given mint supply and realm config
    fn get_max_vote_weight(
        &mut self,
        realm_data: &Realm,
        governing_token_mint_supply: u64,
    ) -> Result<u64, ProgramError> {
        // max vote weight fraction is only used for community mint
        if Some(self.governing_token_mint) == realm_data.config.council_mint {
            return Ok(governing_token_mint_supply);
        }

        match realm_data.config.community_mint_max_vote_weight_source {
            MintMaxVoteWeightSource::SupplyFraction(fraction) => {
                if fraction == MintMaxVoteWeightSource::SUPPLY_FRACTION_BASE {
                    return Ok(governing_token_mint_supply);
                }

                let max_vote_weight = (governing_token_mint_supply as u128)
                    .checked_mul(fraction as u128)
                    .unwrap()
                    .checked_div(MintMaxVoteWeightSource::SUPPLY_FRACTION_BASE as u128)
                    .unwrap() as u64;

                let deny_vote_weight = self.deny_vote_weight.unwrap_or(0);

                let max_option_vote_weight =
                    self.options.iter().map(|o| o.vote_weight).max().unwrap();

                // When the fraction is used it's possible we can go over the calculated max_vote_weight
                // and we have to adjust it in case more votes have been cast
                let total_vote_weight = max_option_vote_weight
                    .checked_add(deny_vote_weight)
                    .unwrap();

                Ok(max_vote_weight.max(total_vote_weight))
            }
            MintMaxVoteWeightSource::Absolute(_) => {
                Err(ShihonError::VoteWeightSourceNotSupported.into())
            }
        }
    }

    /// Checks if vote can be tipped and automatically transitioned to Succeeded or Defeated state
    /// If the conditions are met the state is updated accordingly
    pub fn try_tip_vote(
        &mut self,
        governing_token_mint_supply: u64,
        config: &GovernanceConfig,
        realm_data: &Realm,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<bool, ProgramError> {
        let max_vote_weight = self.get_max_vote_weight(realm_data, governing_token_mint_supply)?;

        if let Some(tipped_state) = self.try_get_tipped_vote_state(max_vote_weight, config) {
            self.state = tipped_state;
            self.voting_completed_at = Some(current_unix_timestamp);

            // Capture vote params to correctly display historical results
            self.max_vote_weight = Some(max_vote_weight);
            self.vote_threshold_percentage = Some(config.vote_threshold_percentage.clone());

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Checks if vote can be tipped and automatically transitioned to Succeeded or Defeated state
    /// If yes then Some(ProposalState) is returned and None otherwise
    #[allow(clippy::float_cmp)]
    pub fn try_get_tipped_vote_state(
        &mut self,
        max_vote_weight: u64,
        config: &GovernanceConfig,
    ) -> Option<ProposalState> {
        // Vote tipping is currently supported for SingleChoice votes with single Yes and No (rejection) options only
        // Note: Tipping for multiple options (single choice and multiple choices) should be possible but it requires a great deal of considerations
        //       and I decided to fight it another day
        if self.vote_type != VoteType::SingleChoice
        // Tipping should not be allowed for opinion only proposals (surveys without rejection) to allow everybody's voice to be heard
        || self.deny_vote_weight.is_none()
        || self.options.len() != 1
        {
            return None;
        };

        let mut yes_option = &mut self.options[0];

        let yes_vote_weight = yes_option.vote_weight;
        let deny_vote_weight = self.deny_vote_weight.unwrap();

        if yes_vote_weight == max_vote_weight {
            yes_option.vote_result = OptionVoteResult::Succeeded;
            return Some(ProposalState::Succeeded);
        }

        if deny_vote_weight == max_vote_weight {
            yes_option.vote_result = OptionVoteResult::Defeated;
            return Some(ProposalState::Defeated);
        }

        let min_vote_threshold_weight =
            get_min_vote_threshold_weight(&config.vote_threshold_percentage, max_vote_weight)
                .unwrap();

        if yes_vote_weight >= min_vote_threshold_weight
            && yes_vote_weight > (max_vote_weight - yes_vote_weight)
        {
            yes_option.vote_result = OptionVoteResult::Succeeded;
            return Some(ProposalState::Succeeded);
        } else if deny_vote_weight > (max_vote_weight - min_vote_threshold_weight)
            || deny_vote_weight >= (max_vote_weight - deny_vote_weight)
        {
            yes_option.vote_result = OptionVoteResult::Defeated;
            return Some(ProposalState::Defeated);
        }

        None
    }

    /// Checks if Proposal can be canceled in the given state
    pub fn assert_can_cancel(
        &self,
        config: &GovernanceConfig,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        match self.state {
            ProposalState::Draft | ProposalState::SigningOff => Ok(()),
            ProposalState::Voting => {
                // Note: If there is no tipping point the proposal can be still in Voting state but already past the configured max_voting_time
                // In that case we treat the proposal as finalized and it's no longer allowed to be canceled
                if self.has_vote_time_ended(config, current_unix_timestamp) {
                    return Err(ShihonError::ProposalVotingTimeExpired.into());
                }
                Ok(())
            }
            ProposalState::Executing
            | ProposalState::ExecutingWithErrors
            | ProposalState::Completed
            | ProposalState::Cancelled
            | ProposalState::Succeeded
            | ProposalState::Defeated => {
                Err(ShihonError::InvalidStateCannotCancelProposal.into())
            }
        }
    }

    /// Checks if Instructions can be edited (inserted or removed) for the Proposal in the given state
    /// It also asserts whether the Proposal is executable (has the reject option)
    pub fn assert_can_edit_instructions(&self) -> Result<(), ProgramError> {
        if self.assert_is_draft_state().is_err() {
            return Err(ShihonError::InvalidStateCannotEditInstructions.into());
        }

        // For security purposes only proposals with the reject option can have executable instructions
        if self.deny_vote_weight.is_none() {
            return Err(ShihonError::ProposalIsNotExecutable.into());
        }

        Ok(())
    }

    /// Checks if Instructions can be executed for the Proposal in the given state
    pub fn assert_can_execute_instruction(
        &self,
        proposal_instruction_data: &ProposalInstructionV2,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        match self.state {
            ProposalState::Succeeded
            | ProposalState::Executing
            | ProposalState::ExecutingWithErrors => {}
            ProposalState::Draft
            | ProposalState::SigningOff
            | ProposalState::Completed
            | ProposalState::Voting
            | ProposalState::Cancelled
            | ProposalState::Defeated => {
                return Err(ShihonError::InvalidStateCannotExecuteInstruction.into())
            }
        }

        if self.options[proposal_instruction_data.option_index as usize].vote_result
            != OptionVoteResult::Succeeded
        {
            return Err(ShihonError::CannotExecuteDefeatedOption.into());
        }

        if self
            .voting_completed_at
            .unwrap()
            .checked_add(proposal_instruction_data.hold_up_time as i64)
            .unwrap()
            >= current_unix_timestamp
        {
            return Err(ShihonError::CannotExecuteInstructionWithinHoldUpTime.into());
        }

        if proposal_instruction_data.executed_at.is_some() {
            return Err(ShihonError::InstructionAlreadyExecuted.into());
        }

        Ok(())
    }

    /// Checks if the instruction can be flagged with error for the Proposal in the given state
    pub fn assert_can_flag_instruction_error(
        &self,
        proposal_instruction_data: &ProposalInstructionV2,
        current_unix_timestamp: UnixTimestamp,
    ) -> Result<(), ProgramError> {
        // Instruction can be flagged for error only when it's eligible for execution
        self.assert_can_execute_instruction(proposal_instruction_data, current_unix_timestamp)?;

        if proposal_instruction_data.execution_status == InstructionExecutionStatus::Error {
            return Err(ShihonError::InstructionAlreadyFlaggedWithError.into());
        }

        Ok(())
    }

    /// Asserts the given vote is valid for the proposal
    pub fn assert_valid_vote(&self, vote: &Vote) -> Result<(), ProgramError> {
        match vote {
            Vote::Approve(choices) => {
                if self.options.len() != choices.len() {
                    return Err(ShihonError::InvalidVote.into());
                }

                let mut choice_count = 0u16;

                for choice in choices {
                    if choice.rank > 0 {
                        return Err(ShihonError::InvalidVote.into());
                    }

                    if choice.weight_percentage == 100 {
                        choice_count = choice_count.checked_add(1).unwrap();
                    } else if choice.weight_percentage != 0 {
                        return Err(ShihonError::InvalidVote.into());
                    }
                }

                match self.vote_type {
                    VoteType::SingleChoice => {
                        if choice_count != 1 {
                            return Err(ShihonError::InvalidVote.into());
                        }
                    }
                    VoteType::MultiChoice(_n) => {
                        if choice_count == 0 {
                            return Err(ShihonError::InvalidVote.into());
                        }
                    }
                }
            }
            Vote::Deny => {
                if self.deny_vote_weight.is_none() {
                    return Err(ShihonError::InvalidVote.into());
                }
            }
        }

        Ok(())
    }

    /// Serializes account into the target buffer
    pub fn serialize<W: Write>(self, writer: &mut W) -> Result<(), ProgramError> {
        if self.account_type == ShihonAccountType::ProposalV2 {
            BorshSerialize::serialize(&self, writer)?
        } else if self.account_type == ShihonAccountType::ProposalV1 {
            // V1 account can't be resized and we have to translate it back to the original format

            let proposal_data_v1 = ProposalV1 {
                account_type: self.account_type,
                governance: self.governance,
                governing_token_mint: self.governing_token_mint,
                state: self.state,
                token_owner_record: self.token_owner_record,
                signatories_count: self.signatories_count,
                signatories_signed_off_count: self.signatories_signed_off_count,
                yes_votes_count: self.options[0].vote_weight,
                no_votes_count: self.deny_vote_weight.unwrap(),
                instructions_executed_count: self.options[0].instructions_executed_count,
                instructions_count: self.options[0].instructions_count,
                instructions_next_index: self.options[0].instructions_next_index,
                draft_at: self.draft_at,
                signing_off_at: self.signing_off_at,
                voting_at: self.voting_at,
                voting_at_slot: self.voting_at_slot,
                voting_completed_at: self.voting_completed_at,
                executing_at: self.executing_at,
                closed_at: self.closed_at,
                execution_flags: self.execution_flags,
                max_vote_weight: self.max_vote_weight,
                vote_threshold_percentage: self.vote_threshold_percentage,
                name: self.name,
                description_link: self.description_link,
            };

            BorshSerialize::serialize(&proposal_data_v1, writer)?;
        }

        Ok(())
    }
}

/// Converts threshold in percentages to actual vote weight
/// and returns the min weight required for a proposal option to pass
fn get_min_vote_threshold_weight(
    vote_threshold_percentage: &VoteThresholdPercentage,
    max_vote_weight: u64,
) -> Result<u64, ProgramError> {
    let yes_vote_threshold_percentage = match vote_threshold_percentage {
        VoteThresholdPercentage::YesVote(yes_vote_threshold_percentage) => {
            *yes_vote_threshold_percentage
        }
        _ => {
            return Err(ShihonError::VoteThresholdPercentageTypeNotSupported.into());
        }
    };

    let numerator = (yes_vote_threshold_percentage as u128)
        .checked_mul(max_vote_weight as u128)
        .unwrap();

    let mut yes_vote_threshold = numerator.checked_div(100).unwrap();

    if yes_vote_threshold.checked_mul(100).unwrap() < numerator {
        yes_vote_threshold = yes_vote_threshold.checked_add(1).unwrap();
    }

    Ok(yes_vote_threshold as u64)
}

/// Deserializes Proposal account and checks owner program
pub fn get_proposal_data(
    program_id: &Pubkey,
    proposal_info: &AccountInfo,
) -> Result<ProposalV2, ProgramError> {
    let account_type: ShihonAccountType =
        try_from_slice_unchecked(&proposal_info.data.borrow())?;

    // If the account is V1 version then translate to V2
    if account_type == ShihonAccountType::ProposalV1 {
        let proposal_data_v1 = get_account_data::<ProposalV1>(program_id, proposal_info)?;

        let vote_result = match proposal_data_v1.state {
            ProposalState::Draft
            | ProposalState::SigningOff
            | ProposalState::Voting
            | ProposalState::Cancelled => OptionVoteResult::None,
            ProposalState::Succeeded
            | ProposalState::Executing
            | ProposalState::ExecutingWithErrors
            | ProposalState::Completed => OptionVoteResult::Succeeded,
            ProposalState::Defeated => OptionVoteResult::None,
        };

        return Ok(ProposalV2 {
            account_type,
            governance: proposal_data_v1.governance,
            governing_token_mint: proposal_data_v1.governing_token_mint,
            state: proposal_data_v1.state,
            token_owner_record: proposal_data_v1.token_owner_record,
            signatories_count: proposal_data_v1.signatories_count,
            signatories_signed_off_count: proposal_data_v1.signatories_signed_off_count,
            vote_type: VoteType::SingleChoice,
            options: vec![ProposalOption {
                label: "Yes".to_string(),
                vote_weight: proposal_data_v1.yes_votes_count,
                vote_result,
                instructions_executed_count: proposal_data_v1.instructions_executed_count,
                instructions_count: proposal_data_v1.instructions_count,
                instructions_next_index: proposal_data_v1.instructions_next_index,
            }],
            deny_vote_weight: Some(proposal_data_v1.no_votes_count),
            draft_at: proposal_data_v1.draft_at,
            signing_off_at: proposal_data_v1.signing_off_at,
            voting_at: proposal_data_v1.voting_at,
            voting_at_slot: proposal_data_v1.voting_at_slot,
            voting_completed_at: proposal_data_v1.voting_completed_at,
            executing_at: proposal_data_v1.executing_at,
            closed_at: proposal_data_v1.closed_at,
            execution_flags: proposal_data_v1.execution_flags,
            max_vote_weight: proposal_data_v1.max_vote_weight,
            vote_threshold_percentage: proposal_data_v1.vote_threshold_percentage,
            name: proposal_data_v1.name,
            description_link: proposal_data_v1.description_link,
        });
    }

    get_account_data::<ProposalV2>(program_id, proposal_info)
}

/// Deserializes Proposal and validates it belongs to the given Governance and Governing Mint
pub fn get_proposal_data_for_governance_and_governing_mint(
    program_id: &Pubkey,
    proposal_info: &AccountInfo,
    governance: &Pubkey,
    governing_token_mint: &Pubkey,
) -> Result<ProposalV2, ProgramError> {
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
) -> Result<ProposalV2, ProgramError> {
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
    vote_type: &VoteType,
) -> Result<(), ProgramError> {
    if options.is_empty() {
        return Err(ShihonError::InvalidProposalOptions.into());
    }

    if let VoteType::MultiChoice(n) = *vote_type {
        if options.len() == 1 || n as usize != options.len() {
            return Err(ShihonError::InvalidProposalOptions.into());
        }
    }

    if options.iter().any(|o| o.is_empty()) {
        return Err(ShihonError::InvalidProposalOptions.into());
    }

    Ok(())
}

