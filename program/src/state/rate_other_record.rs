//! Signatory Record

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{error::ShihonError, PROGRAM_AUTHORITY_SEED};

use crate::state::enums::ShihonAccountType;

/// Account PDA seeds: ['shihon', rate, signatory]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct RateOtherRecord {
    ///
    pub is_initialized: bool,

    /// Rate account type
    pub account_type: ShihonAccountType,

    /// Mix content record
    pub mix_content_record: Pubkey,

    // CC vote record
    pub cc_vote_record: Pubkey,

    /// Outside Buyer Record
    pub outside_buyer_record: Pubkey,

    /// Rate count
    pub rate_amount: u64,

    /// Rating time
    pub rating_time: Option<UnixTimestamp>,
}

impl AccountMaxSize for RateOtherRecord {}

impl IsInitialized for RateOtherRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::RateOtherRecord
    }
}

impl RateOtherRecord {
    /// Checks signatory hasn't signed off yet and is transaction signer
    pub fn assert_can_rate(&self, rating_info: &AccountInfo) -> Result<(), ProgramError> {
        if self.is_initialized {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        if !rating_info.rater_pubkey {
            return Err(ShihonError::SignatoryMustSign.into());
        }

        Ok(())
    }

    /// Checks signatory can be removed from Proposal
    pub fn assert_can_remove_rating(&self) -> Result<(), ProgramError> {
        if self.is_initialized {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        Ok(())
    }
}

/// Returns RateOtherRecord PDA seeds
pub fn get_rate_other_record_address_seeds<'a>(
    rater_pubkey: &'a Pubkey,
    rated_content: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        rater_pubkey.as_ref(),
        rated_content.as_ref(),
    ]
}

/// Returns RateOtherRecord PDA address
pub fn get_rate_other_record_address<'a>(
    program_id: &Pubkey,
    rater_pubkey: &'a Pubkey,
    rated_content: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_rate_other_record_address_seeds(rater_pubkey, rated_content),
        program_id,
    )
    .0
}

/// Deserializes RateOtherRecord account and checks owner program
pub fn get_rate_other_record_data(
    program_id: &Pubkey,
    rate_other_record_info: &AccountInfo,
) -> Result<RateOtherRecord, ProgramError> {
    get_account_data::<RateOtherRecord>(program_id, rate_other_record_info)
}

/// Deserializes RateOtherRecord  and validates its PDA
pub fn get_rete_other_record_data_for_seeds(
    program_id: &Pubkey,
    rate_other_record_info: &AccountInfo,
    rater_pubkey: &Pubkey,
    rated_content: &Pubkey,
) -> Result<RateOtherRecord, ProgramError> {
    let (rate_other_record_address, _) = Pubkey::find_program_address(
        &get_rate_other_record_address_seeds(rated_content, rater_pubkey),
        program_id,
    );

    if rate_other_record_address != *rate_other_record_info.key {
        return Err(ShihonError::InvalidSignatoryAddress.into());
    }

    get_rate_other_record_data(program_id, rate_other_record_info)
}
