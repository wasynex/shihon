//! bcToken Metadata Account

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::Slot, program_error::ProgramError,
    program_pack::IsInitialized, pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::state::enums::ShihonAccountType;

/// bcToken metadata account
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcTokenMetadata {
    /// bcToken account type
    pub account_type: ShihonAccountType,

    /// what content type
    pub content_type: ContentType,

    /// The slot when the metadata was captured
    pub updated_at: Slot,

    /// The version of the bcToken
    pub version: String,

    /// Reserved
    pub reserved: [u8; 64],

    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,

    /// True if an bcToken requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
}

/// The content type of bcToken
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ContentType {
    /// order content mix type
    Ordinary,

    /// partial content mix type
    Partially,

    /// other content mix type
    Other,
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

impl BcTokenMetadata {
    pub fn assert_can_poking_content() {
        unimplemented!();
    }

    pub fn assert_is_valid_content_type_matched() {
        unimplemented!();
    }

    pub fn assert_is_valid_received_link_from_oracle() {
        unimplemented!();
    }

    pub fn assert_is_valid_hold_content_data_in_bc_token() {
        unimplemented!();
    }

    pub fn assert_is_correct_owner() {
        unimplemented!();
    }

    pub fn get_which_content_version() {
        unimplemented!();
    }

    pub fn assert_is_valid_public() {
        unimplemented!();
    }

    pub fn assert_can_reach_each_content_on_oracle() {
        unimplemented!();
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
