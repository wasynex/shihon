//! Candidate Limit Record Account

use std::slice::Iter;

use crate::{
    addins::voter_weight::{
        get_voter_weight_record_data_for_token_owner_record, VoterWeightAction,
    },
    error::ShihonError,
    state::{
        enums::ShihonAccountType, governance::GovernanceConfig, realm::Realm,
        realm_config::get_realm_config_data_for_realm,
    },
    PROGRAM_AUTHORITY_SEED,
};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

/// Account PDA seeds: ['shihon', bcToken, token_mint, token_owner ]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct CandidateLimitRecord {
    /// Governance account type
    pub account_type: ShihonAccountType,

    /// The Tanistry the CandidateLimitRecord belongs to
    pub tanistry: Pubkey,

    /// number of candidate
    pub number_of_candidate: u32,

    /// Candidate Token Mint the CandidateLimitRecord holds deposit for
    pub candidate_token_mint: Pubkey,

    /// The owner (either single or multisig) of the deposited candidate SPL Tokens
    /// This is who can authorize a withdrawal of the tokens
    pub candidate_token_owner: Pubkey,

    /// The amount of candidate tokens deposited into the Tanistry
    /// This is to use it as self-rating
    pub candidate_token_deposit_amount: u64,

    /// candidate(bcToken)
    pub candidate: Pubkey,
}

impl AccountMaxSize for CandidateLimitRecord {
    fn get_max_size(&self) -> Option<usize> {
        Some(154)
    }
}

impl IsInitialized for CandidateLimitRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::CandidateLimitRecord
    }
}
///TODO: not yet fix this associated functions
/// we need to write the Limit code here
impl CandidateLimitRecord {
    /// Checks whether the provided Governance Authority signed transaction
    pub fn assert_token_owner_or_delegate_is_signer(
        &self,
        governance_authority_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if governance_authority_info.is_signer {
            if &self.candidate_token_owner == governance_authority_info.key {
                return Ok(());
            }

            if let Some(candidate_delegate) = self.candidate_delegate {
                if &governance_delegate == governance_authority_info.key {
                    return Ok(());
                }
            };
        }

        Err(ShihonError::GoverningTokenOwnerOrDelegateMustSign.into())
    }

    /// Asserts TokenOwner has enough tokens to be allowed to create proposal and doesn't have any outstanding proposals
    pub fn assert_can_create_proposal(
        &self,
        realm_data: &Realm,
        config: &GovernanceConfig,
        voter_weight: u64,
    ) -> Result<(), ProgramError> {
        let min_weight_to_create_proposal =
            if self.governing_token_mint == realm_data.community_mint {
                config.min_community_tokens_to_create_proposal
            } else if Some(self.governing_token_mint) == realm_data.config.council_mint {
                config.min_council_tokens_to_create_proposal
            } else {
                return Err(ShihonError::InvalidGoverningTokenMint.into());
            };

        if voter_weight < min_weight_to_create_proposal {
            return Err(ShihonError::NotEnoughTokensToCreateProposal.into());
        }

        // The number of outstanding proposals is currently restricted to 10
        // If there is a need to change it in the future then it should be added to realm or governance config
        if self.outstanding_proposal_count >= 10 {
            return Err(ShihonError::TooManyOutstandingProposals.into());
        }

        Ok(())
    }

    /// Asserts TokenOwner has enough tokens to be allowed to create governance
    pub fn assert_can_create_governance(
        &self,
        realm_data: &Realm,
        voter_weight: u64,
    ) -> Result<(), ProgramError> {
        let min_weight_to_create_governance =
            if self.governing_token_mint == realm_data.community_mint {
                realm_data.config.min_community_tokens_to_create_governance
            } else if Some(self.governing_token_mint) == realm_data.config.council_mint {
                // For council tokens it's enough to be in possession of any number of tokens
                1
            } else {
                return Err(ShihonError::InvalidGoverningTokenMint.into());
            };

        if voter_weight < min_weight_to_create_governance {
            return Err(ShihonError::NotEnoughTokensToCreateGovernance.into());
        }

        Ok(())
    }

    /// Asserts TokenOwner can withdraw tokens from Realm
    pub fn assert_can_withdraw_governing_tokens(&self) -> Result<(), ProgramError> {
        if self.unrelinquished_votes_count > 0 {
            return Err(ShihonError::AllVotesMustBeRelinquishedToWithdrawGoverningTokens.into());
        }

        if self.outstanding_proposal_count > 0 {
            return Err(ShihonError::AllProposalsMustBeFinalisedToWithdrawGoverningTokens.into());
        }

        Ok(())
    }

    /// Decreases outstanding_proposal_count
    pub fn decrease_outstanding_proposal_count(&mut self) {
        // Previous versions didn't use the count and it can be already 0
        if self.outstanding_proposal_count != 0 {
            self.outstanding_proposal_count =
                self.outstanding_proposal_count.checked_sub(1).unwrap();
        }
    }

    /// Resolves voter's weight using either the amount deposited into the realm or weight provided by voter weight addin (if configured)
    pub fn resolve_voter_weight(
        &self,
        program_id: &Pubkey,
        account_info_iter: &mut Iter<AccountInfo>,
        realm: &Pubkey,
        realm_data: &Realm,
        weight_action: VoterWeightAction,
        weight_action_target: &Pubkey,
    ) -> Result<u64, ProgramError> {
        // if the realm uses addin for community voter weight then use the externally provided weight
        if realm_data.config.use_community_voter_weight_addin
            && realm_data.community_mint == self.governing_token_mint
        {
            let realm_config_info = next_account_info(account_info_iter)?;
            let voter_weight_record_info = next_account_info(account_info_iter)?;

            let realm_config_data =
                get_realm_config_data_for_realm(program_id, realm_config_info, realm)?;

            let voter_weight_record_data = get_voter_weight_record_data_for_token_owner_record(
                &realm_config_data.community_voter_weight_addin.unwrap(),
                voter_weight_record_info,
                self,
            )?;
            voter_weight_record_data
                .assert_is_valid_voter_weight(weight_action, weight_action_target)?;
            Ok(voter_weight_record_data.voter_weight)
        } else {
            Ok(self.governing_token_deposit_amount)
        }
    }
}

/// Returns CandidateLimitRecord PDA address
pub fn get_candidate_limit_record_address(
    program_id: &Pubkey,
    tanistry: &Pubkey,
    candidate_token_mint: &Pubkey,
    candidate_token_owner: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_candidate_limit_record_address_seeds(
            tanistry,
            candidate_token_mint,
            candidate_token_owner,
        ),
        program_id,
    )
    .0
}

/// Returns CandidateLimitRecord PDA seeds
pub fn get_candidate_limit_record_address_seeds<'a>(
    tanistry: &'a Pubkey,
    candidate_token_mint: &'a Pubkey,
    candidate_token_owner: &'a Pubkey,
) -> [&'a [u8]; 4] {
    [
        PROGRAM_AUTHORITY_SEED,
        tanistry.as_ref(),
        candidate_token_mint.as_ref(),
        candidate_token_owner.as_ref(),
    ]
}

/// Deserializes CandidateLimitRecord account and checks owner program
pub fn get_candidate_limit_record_data(
    program_id: &Pubkey,
    candidate_limit_record_info: &AccountInfo,
) -> Result<CandidateLimitRecord, ProgramError> {
    get_account_data::<CandidateLimitRecord>(program_id, candidate_limit_record_info)
}

/// Deserializes CandidateLimitRecord account and checks its PDA against the provided seeds
pub fn get_candidate_limit_record_data_for_seeds(
    program_id: &Pubkey,
    candidate_limit_record_info: &AccountInfo,
    candidate_limit_record_seeds: &[&[u8]],
) -> Result<CandidateLimitRecord, ProgramError> {
    let (candidate_limit_record_address, _) =
        Pubkey::find_program_address(candidate_limit_record_seeds, program_id);

    if candidate_limit_record_address != *candidate_limit_record_info.key {
        return Err(ShihonError::InvalidTokenOwnerRecordAccountAddress.into());
    }

    get_candidate_limit_record_data(program_id, candidate_limit_record_info)
}

/// Deserializes CandidateLimitRecord account and asserts it belongs to the given Tanistry
pub fn get_candidate_limit_record_data_for_tanistry(
    program_id: &Pubkey,
    candidate_limit_record_info: &AccountInfo,
    tanistry: &Pubkey,
) -> Result<CandidateLimitRecord, ProgramError> {
    let candidate_limit_record_data =
        get_candidate_limit_record_data(program_id, candidate_limit_record_info)?;

    if candidate_limit_record_data.tanistry != *tanistry {
        return Err(ShihonError::InvalidRealmForTokenOwnerRecord.into());
    }

    Ok(candidate_limit_record_data)
}

/// Deserializes CandidateLimitRecord account and  asserts it belongs to the given Tanistry and is for the given candidate mint
pub fn get_candidate_limit_record_data_for_tanistry_and_candidate_mint(
    program_id: &Pubkey,
    candidate_limit_record_info: &AccountInfo,
    tanistry: &Pubkey,
    candidate_token_mint: &Pubkey,
) -> Result<CandidateLimitRecord, ProgramError> {
    let candidate_limit_record_data = get_candidate_limit_record_data_for_tanistry(
        program_id,
        candidate_limit_record_info,
        tanistry,
    )?;

    if candidate_limit_record_data.candidate_token_mint != *candidate_token_mint {
        return Err(ShihonError::InvalidGoverningMintForTokenOwnerRecord.into());
    }

    Ok(candidate_limit_record_data)
}

///  Deserializes CandidateLimitRecord account and checks its address is the give rater
pub fn get_candidate_limit_record_data_for_rater(
    program_id: &Pubkey,
    candidate_limit_record_info: &AccountInfo,
    rater: &Pubkey,
) -> Result<CandidateLimitRecord, ProgramError> {
    if candidate_limit_record_info.key != rater {
        return Err(ShihonError::InvalidProposalOwnerAccount.into());
    }

    get_candidate_limit_record_data(program_id, candidate_limit_record_info)
}
