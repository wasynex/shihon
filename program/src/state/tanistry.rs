//! Tanistry Account

use crate::{
    error::ShihonError,
    state::{
        enums::{ShihonAccountType, VoteThresholdPercentage, VoteWeightSource},
        realm::assert_is_valid_realm,
    },
};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::{
    account::{assert_is_valid_account2, get_account_data, AccountMaxSize},
    error::GovernanceToolsError,
};

/// Tanistry config
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct TanistryConfig {
    /// The type of the vote threshold used for voting
    /// Note: In the current version only YesVote threshold is supported
    pub vote_threshold_percentage: VoteThresholdPercentage,

    /// Minimum number of community tokens a governance token owner must possess to be able to create a proposal
    pub min_community_tokens_to_create_proposal: u64,

    /// Minimum waiting time in seconds for an instruction to be executed after proposal is voted on
    pub min_instruction_hold_up_time: u32,

    /// Time limit in seconds for proposal to be open for voting
    pub max_voting_time: u32,

    /// The source of vote weight for voters
    /// Note: In the current version only token deposits are accepted as vote weight
    pub vote_weight_source: VoteWeightSource,

    /// The time period in seconds within which a Proposal can be still cancelled after being voted on
    /// Once cool off time expires Proposal can't be cancelled any longer and becomes a law
    /// Note: This field is not implemented in the current version
    pub proposal_cool_off_time: u32,

    /// Minimum number of council tokens a governance token owner must possess to be able to create a proposal
    pub min_council_tokens_to_create_proposal: u64,
}

/// Tanistry Account
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct Tanistry {
    pub is_initialized: bool,
    pub kicker_pubkey: Pubkey,
    pub next_tanistry_id: Option<Pubkey>,
    pub previous_tanistry_id: Option<Pubkey>,
    pub building_key: Vec<u8>,

    pub account_type: ShihonAccountType,

    /// Governance config
    pub config: TanistryConfig,

    /// Reserved space for future versions
    pub reserved: [u8; 8],
}

impl AccountMaxSize for Tanistry {}

impl IsInitialized for Tanistry {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::AccountGovernance
            || self.account_type == ShihonAccountType::ProgramGovernance
            || self.account_type == ShihonAccountType::MintGovernance
            || self.account_type == ShihonAccountType::TokenGovernance
    }
}

impl Tanistry {
    /// Returns Tanistry PDA seeds
    pub fn get_tanistry_address_seeds(&self) -> Result<[&[u8]; 3], ProgramError> {
        let seeds = match self.account_type {
            ShihonAccountType::AccountGovernance => {
                get_account_governance_address_seeds(&self.realm, &self.governed_account)
            }
            ShihonAccountType::ProgramGovernance => {
                get_program_governance_address_seeds(&self.realm, &self.governed_account)
            }
            ShihonAccountType::MintGovernance => {
                get_mint_governance_address_seeds(&self.realm, &self.governed_account)
            }
            ShihonAccountType::TokenGovernance => {
                get_token_governance_address_seeds(&self.realm, &self.governed_account)
            }
            _ => return Err(GovernanceToolsError::InvalidAccountType.into()),
        };

        Ok(seeds)
    }
}

/// Deserializes Tanistry account and checks owner program
pub fn get_tanistry_data(
    program_id: &Pubkey,
    tanistry_info: &AccountInfo,
) -> Result<Tanistry, ProgramError> {
    get_account_data::<Tanistry>(program_id, tanistry_info)
}

/// Deserializes Tanistry account, checks owner program and asserts governance belongs to the given ream
pub fn get_tanistry_data_for_bc_token(
    program_id: &Pubkey,
    tanistry_info: &AccountInfo,
    bc_token: &Pubkey,
) -> Result<Tanistry, ProgramError> {
    let tanistry_data = get_tanistry_data(program_id, tanistry_info)?;

    if tanistry_data.bc_token != *bc_token {
        return Err(ShihonError::InvalidRealmForGovernance.into());
    }

    Ok(tanistry_data)
}

/// Checks the given account is a governance account and belongs to the given realm
pub fn assert_governance_for_realm(
    program_id: &Pubkey,
    tanistry_info: &AccountInfo,
    bc_token: &Pubkey,
) -> Result<(), ProgramError> {
    get_tanistry_data_for_bc_token(program_id, tanistry_info, bc_token)?;
    Ok(())
}

/// Returns Tanistry PDA seeds
pub fn get_program_tanistry_address_seeds<'a>(
    bc_token: &'a Pubkey,
    tanistry_program: &'a Pubkey,
) -> [&'a [u8]; 3] {
    // 'program-governance' prefix ensures uniqueness of the PDA
    // Note: Only the current program upgrade authority can create an account with this PDA using CreateProgramGovernance instruction
    [
        b"program-tanistry",
        bc_token.as_ref(),
        tanistry_program.as_ref(),
    ]
}

/// Returns ProgramGovernance PDA address
pub fn get_program_tanistry_address<'a>(
    program_id: &Pubkey,
    bc_token: &'a Pubkey,
    tanistry_program: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_program_tanistry_address_seeds(bc_token, tanistry_program),
        program_id,
    )
    .0
}

/// Returns MintTanistry PDA seeds
pub fn get_mint_tanistry_address_seeds<'a>(
    bc_token: &'a Pubkey,
    tanistry_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    // 'mint-governance' prefix ensures uniqueness of the PDA
    // Note: Only the current mint authority can create an account with this PDA using CreateMintGovernance instruction
    [b"mint-tanistry", bc_token.as_ref(), tanistry_mint.as_ref()]
}

/// Returns MintTanistry PDA address
pub fn get_mint_tanistry_address<'a>(
    program_id: &Pubkey,
    bc_token: &'a Pubkey,
    tanistry_mint: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_mint_tanistry_address_seeds(bc_token, tanistry_mint),
        program_id,
    )
    .0
}

/// Returns TokenGovernance PDA seeds
pub fn get_token_tanistry_address_seeds<'a>(
    bc_token: &'a Pubkey,
    tanistry_token: &'a Pubkey,
) -> [&'a [u8]; 3] {
    // 'token-tanistry' prefix ensures uniqueness of the PDA
    // Note: Only the current token account owner can create an account with this PDA using CreateTokenTanistry instruction
    [
        b"token-tanistry",
        bc_token.as_ref(),
        tanistry_token.as_ref(),
    ]
}

/// Returns TokenTanistry PDA address
pub fn get_token_tanistry_address<'a>(
    program_id: &Pubkey,
    realm: &'a Pubkey,
    governed_token: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_token_tanistry_address_seeds(realm, governed_token),
        program_id,
    )
    .0
}

/// Returns AccountTanistry PDA seeds
pub fn get_account_tanistry_address_seeds<'a>(
    realm: &'a Pubkey,
    governed_account: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        b"account-tanistry",
        realm.as_ref(),
        governed_account.as_ref(),
    ]
}

/// Returns AccountTanistry PDA address
pub fn get_account_tanistry_address<'a>(
    program_id: &Pubkey,
    realm: &'a Pubkey,
    governed_account: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_account_tanistry_address_seeds(realm, governed_account),
        program_id,
    )
    .0
}

/// Checks whether tanistry account exists, is initialized and owned by the Tanistry program
pub fn assert_is_valid_tanistry(
    program_id: &Pubkey,
    tanistry_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_is_valid_account2(
        tanistry_info,
        &[
            ShihonAccountType::AccountGovernance,
            ShihonAccountType::ProgramGovernance,
            ShihonAccountType::TokenGovernance,
            ShihonAccountType::MintGovernance,
        ],
        program_id,
    )
}

/// Validates args supplied to create tanistry account
pub fn assert_valid_create_tanistry_args(
    program_id: &Pubkey,
    tanistry_config: &TanistryConfig,
    bc_token_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_is_valid_bc_token(program_id, bc_token_info)?;

    assert_is_valid_tanistry_config(tanistry_config)?;

    Ok(())
}

/// Validates tanistry config parameters
pub fn assert_is_valid_tanistry_config(
    tanistry_config: &TanistryConfig,
) -> Result<(), ProgramError> {
    match tanistry_config.vote_threshold_percentage {
        VoteThresholdPercentage::YesVote(yes_vote_threshold_percentage) => {
            if !(1..=100).contains(&yes_vote_threshold_percentage) {
                return Err(ShihonError::InvalidVoteThresholdPercentage.into());
            }
        }
        _ => {
            return Err(ShihonError::VoteThresholdPercentageTypeNotSupported.into());
        }
    }

    if tanistry_config.vote_weight_source != VoteWeightSource::Deposit {
        return Err(ShihonError::VoteWeightSourceNotSupported.into());
    }

    if tanistry_config.proposal_cool_off_time > 0 {
        return Err(ShihonError::ProposalCoolOffTimeNotSupported.into());
    }

    Ok(())
}
