//! bcToken Metadata Account

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::Slot, program_error::ProgramError,
    program_pack::IsInitialized, pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::state::enums::{ContentType, ShihonAccountType};

/// Program metadata account. It stores information about the particular SPL-Governance program instance
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcTokenMetadata {
    /// bcToken account type
    pub account_type: ShihonAccountType,

    /// The slot when the metadata was captured
    pub updated_at: Slot,

    /// The version of the bcToken
    pub version: String,

    /// Reserved
    pub reserved: [u8; 64],

    /// what content type
    pub content_type: ContentType,
}

/// bcToken Config instruction args
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcTokenConfigArgs {
    /// Indicates whether council_mint should be used
    /// If yes then council_mint account must also be passed to the instruction
    pub use_council_mint: bool,

    /// Min number of community tokens required to create a governance
    pub min_community_tokens_to_create_governance: u64,

    /// The source used for community mint max vote weight source
    pub community_mint_max_vote_weight_source: MintMaxVoteWeightSource,

    /// Indicates whether an external addin program should be used to provide community voters weights
    /// If yes then the voters weight program account must be passed to the instruction
    pub use_community_voter_weight_addin: bool,
}

/// Realm Config defining Realm parameters.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcTokenConfig {
    /// Indicates whether an external addin program should be used to provide voters weights for the community mint
    pub use_community_voter_weight_addin: bool,

    /// Reserved space for future versions
    pub reserved: [u8; 7],

    /// Min number of community tokens required to create a governance
    pub min_community_tokens_to_create_governance: u64,

    /// The source used for community mint max vote weight source
    pub community_mint_max_vote_weight_source: MintMaxVoteWeightSource,

    /// for candidate
    pub bc_token_mint: Pubkey,
}

impl AccountMaxSize for BcTokenMetadata {
    fn get_max_size(&self) -> Option<usize> {
        Some(88)
    }
}

impl IsInitialized for BcTokenMetadata {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::BcTokenMetadata
    }
}

/// Returns bcToken Metadata PDA address
pub fn get_bc_token_metadata_address(program_id: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_bc_token_metadata_seeds(), program_id).0
}

/// Returns bcToken Metadata PDA seeds
pub fn get_bc_token_metadata_seeds<'a>() -> [&'a [u8]; 1] {
    [b"metadata"]
}

/// Deserializes account and checks owner bcToken
pub fn get_bc_token_metadata_data(
    program_id: &Pubkey,
    bc_token_metadata_info: &AccountInfo,
) -> Result<BcTokenMetadata, ProgramError> {
    get_account_data::<BcTokenMetadata>(program_id, bc_token_metadata_info)
}
