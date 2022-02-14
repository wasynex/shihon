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
    pub belonging_tanistry: Pubkey,

    /// number of candidate count
    ///Lamport envisioned a bakery with a numbering machine at its entrance so each candidate is given a unique number.
    pub number_of_candidate_count: u32,

    /// Candidate Token Mint the CandidateLimitRecord holds deposit for
    pub candidate_token_mint: Pubkey,

    /// The owner (either single or multisig) of the deposited candidate SPL Tokens
    /// This is who can authorize a withdrawal of the tokens
    pub candidate_token_owner: Pubkey,

    /// The amount of candidate tokens deposited into the Tanistry
    /// How much pay did candidate as self-rating
    pub candidate_token_deposit_amount: u64,
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

impl CandidateLimitRecord {
    /// Checks whether the provided bcToken Authority signed transaction
    pub fn assert_token_owner_or_delegate_is_signer(
        &self,
        candidate_token_authority_info: &AccountInfo,
    ) -> Result<(), ProgramError> {
        if candidate_token_authority_info.is_signer {
            if &self.candidate_token_owner == candidate_token_authority_info.key {
                return Ok(());
            };
        }

        Err(ShihonError::CandidateTokenOwnerMustSign.into())
    }

    /// TODO: we need to write the Limit Bar's code here.
    /// Asserts TokenOwner has enough tokens to be allowed to candidate
    pub fn assert_can_create_candidate_limit_record(
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

    /// these funcs moved from modules/index.rs
    fn get_candidate_limit_bar() -> bool {
        unimplemented!();
    }

    /// and here.
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

    /// Increases number_of_candidate_count
    pub fn increase_number_of_candidate_count(&self) {
        if self.number_of_candidate_count != 0 {
            self.number_of_candidate_count = self.number_of_candidate_count.checked_sub(1).unwrap();
        }
    }

    /// no need func now
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

    if candidate_limit_record_data.belonging_tanistry != *tanistry {
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
