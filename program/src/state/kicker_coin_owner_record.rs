//! KickerCoin Owner Record

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{error::ShihonError, PROGRAM_AUTHORITY_SEED};

use crate::state::enums::ShihonAccountType;

/// KickerCoin Owner Record PDA seeds: ['shihon', kicker_coin_holder, coordinator]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct KickerCoinOwnerRecord {
    /// account type
    pub account_type: ShihonAccountType,

    /// KickerCoin holder(first kicker or all crown)
    pub kicker_coin_holder: Pubkey,

    /// The account of the Coordinator, which means the person who receive KickerCoin
    pub coordinator: Pubkey,

    /// Indicates whether the coordinator approve the KickerCoin
    pub is_kick_off: bool,
}

impl AccountMaxSize for KickerCoinOwnerRecord {}

impl IsInitialized for KickerCoinOwnerRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::KickerCoinOwnerRecord
    }
}

impl KickerCoinOwnerRecord {
    /// Checks signatory hasn't signed off yet and is transaction signer
    pub fn assert_can_kick_off(&self, kicker_coin_info: &AccountInfo) -> Result<(), ProgramError> {
        if self.kick_off {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        if !kicker_coin_info.is_signer {
            return Err(ShihonError::SignatoryMustSign.into());
        }

        Ok(())
    }

    /// Checks KickerCoin owner record can be removed from KickerCoinOwnerRecord
    pub fn assert_can_remove_kicker_coin_owner(&self) -> Result<(), ProgramError> {
        if self.kick_off {
            return Err(ShihonError::SignatoryAlreadySignedOff.into());
        }

        Ok(())
    }
}

/// Returns KickerCoinOwnerRecord PDA seeds
pub fn get_kicker_coin_owner_record_address_seeds<'a>(
    kicker_coin_holder: &'a Pubkey,
    coordinator: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        kicker_coin_holder.as_ref(),
        coordinator.as_ref(),
    ]
}

/// Returns KickerCoinOwnerRecord PDA address
pub fn get_kicker_coin_owner_record_address<'a>(
    program_id: &Pubkey,
    kicker_coin_holder: &'a Pubkey,
    coordinator: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_kicker_coin_owner_record_address_seeds(kicker_coin_holder, coordinator),
        program_id,
    )
    .0
}

/// Deserializes KickerCoinOwnerRecord account and checks owner program
pub fn get_kicker_coin_owner_record_data(
    program_id: &Pubkey,
    kicker_coin_owner_record_info: &AccountInfo,
) -> Result<KickerCoinOwnerRecord, ProgramError> {
    get_account_data::<KickerCoinOwnerRecord>(program_id, kicker_coin_owner_record_info)
}

/// Deserializes KickerCoinOwnerRecord  and validates its PDA
pub fn get_kicker_coin_owner_record_data_for_seeds(
    program_id: &Pubkey,
    kicker_coin_owner_record_info: &AccountInfo,
    kicker_coin_holder: &Pubkey,
    coordinator: &Pubkey,
) -> Result<KickerCoinOwnerRecord, ProgramError> {
    let (kicker_coin_owner_record_address, _) = Pubkey::find_program_address(
        &get_kicker_coin_owner_record_address_seeds(kicker_coin_holder, coordinator),
        program_id,
    );

    if kicker_coin_owner_record_address != *kicker_coin_owner_record_info.key {
        return Err(ShihonError::InvalidSignatoryAddress.into());
    }

    get_kicker_coin_owner_record_data(program_id, kicker_coin_owner_record_info)
}
