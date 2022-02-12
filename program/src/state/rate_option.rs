//! Rate Option

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{error::ShihonError, PROGRAM_AUTHORITY_SEED};

use crate::state::enums::ShihonAccountType;

/// Account PDA seeds: ['governance', proposal, signatory]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RateOption {
    /// account type
    pub account_type: ShihonAccountType,

    /// MixContentRecord
    pub mix_content_record: Pubkey,

    /// RateOtherRecord
    pub rate_other_record: Pubkey,

    /// number of issue
    pub number_of_issue: u8,

    /// Key Holder
    pub buddy_candidate: Pubkey,

    pub init_content: Pubkey,
}

impl AccountMaxSize for RateOption {}

impl IsInitialized for RateOption {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::RateOption
    }
}

impl RateOption {
    /// Checks signatory hasn't signed off yet and is transaction signer
    pub fn assert_can_sign_off(&self, signatory_info: &AccountInfo) -> Result<(), ProgramError> {
        if self.signed_off {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        if !signatory_info.is_signer {
            return Err(ShihonError::SignatoryMustSign.into());
        }

        Ok(())
    }

    /// Checks signatory can be removed from Proposal
    pub fn assert_can_remove_signatory(&self) -> Result<(), ProgramError> {
        if self.signed_off {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        Ok(())
    }
}

/// Returns Rate Option PDA seeds
pub fn get_signatory_record_address_seeds<'a>(
    proposal: &'a Pubkey,
    signatory: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        proposal.as_ref(),
        signatory.as_ref(),
    ]
}

/// Returns Rate Option PDA address
pub fn get_signatory_record_address<'a>(
    program_id: &Pubkey,
    proposal: &'a Pubkey,
    signatory: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_signatory_record_address_seeds(proposal, signatory),
        program_id,
    )
    .0
}

/// Deserializes Rate Option account and checks owner program
pub fn get_signatory_record_data(
    program_id: &Pubkey,
    signatory_record_info: &AccountInfo,
) -> Result<RateOption, ProgramError> {
    get_account_data::<RateOption>(program_id, signatory_record_info)
}

/// Deserializes Rate Option  and validates its PDA
pub fn get_signatory_record_data_for_seeds(
    program_id: &Pubkey,
    signatory_record_info: &AccountInfo,
    proposal: &Pubkey,
    signatory: &Pubkey,
) -> Result<RateOption, ProgramError> {
    let (signatory_record_address, _) = Pubkey::find_program_address(
        &get_signatory_record_address_seeds(proposal, signatory),
        program_id,
    );

    if signatory_record_address != *signatory_record_info.key {
        return Err(ShihonError::InvalidSignatoryAddress.into());
    }

    get_signatory_record_data(program_id, signatory_record_info)
}
