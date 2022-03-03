use solana_program::clock::UnixTimestamp;

use crate::state::bc_token_metadata::BcTokenMetadata;
use crate::state::enums::BcTokenState;
use crate::{error::ShihonError, state::enums::ShihonAccountType, PROGRAM_AUTHORITY_SEED};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{assert_is_valid_account, get_account_data, AccountMaxSize};

/// bcToken Account PDA seeds" ['shihon', name]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcToken {
    /// account type
    pub account_type: ShihonAccountType,

    /// bcToken owner pubkey
    pub bc_token_owner_pubkey: Pubkey,

    /// amount of coin (self rating)
    pub amount_of_coin: u64,

    /// bcToken issue time
    pub issue_at: UnixTimestamp,

    /// Reserved space for future versions
    pub reserved: [u8; 8],

    /// bcToken authority
    pub authority: Option<Pubkey>,

    /// bcToken name
    pub name: String,

    /// bcToken Mint
    pub bc_token_mint: Pubkey,

    /// state bcToken
    pub bc_token_state: BcTokenState,

    /// Metadata of bcToken
    pub config: BcTokenMetadata,
}

impl IsInitialized for BcToken {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::BcToken
    }
}

impl AccountMaxSize for BcToken {
    fn get_max_size(&self) -> Option<usize> {
        Some(self.name.len() + 136)
    }
}

impl BcToken {
    /// Asserts the given mint is mint of creating the bcToken
    pub fn assert_is_valid_bc_token_mint(
        &self,
        bc_token_mint: &Pubkey,
    ) -> Result<(), ProgramError> {
        if self.bc_token_mint == *bc_token_mint {
            return Ok(());
        }
        Err(ShihonError::InvalidBcTokenMint.into())
    }

    /// Asserts the given bcToken mint and holding accounts are valid for Tanistry
    pub fn assert_is_valid_bc_token_mint_and_holding(
        &self,
        program_id: &Pubkey,
        bc_token: &Pubkey,
        bc_token_mint: &Pubkey,
        bc_token_holding: &Pubkey,
    ) -> Result<(), ProgramError> {
        self.assert_is_valid_bc_token_mint(bc_token_mint)?;

        let bc_token_holding_address =
            get_bc_token_holding_address(program_id, bc_token, bc_token_mint);

        if bc_token_holding_address != *bc_token_holding {
            return Err(ShihonError::InvalidBcTokenHoldingAccount.into());
        }

        Ok(())
    }

    /// Asserts the given bcToken can be deposited into the Tanistry
    pub fn asset_can_deposits_on_bc_token(
        &self,
        bc_token_mint: &Pubkey,
    ) -> Result<(), ProgramError> {
        ///TODO: create get_bc_token_account_on_tanistry func on tanistry.rs
        // If the deposit is for the bcToken and the token same as on Tanistry using twice then panic
        if self.get_bc_token_account_on_tanistry() && self.bc_token_mint == *bc_token_mint {
            return Err(ShihonError::BcTokenDepositsNotAllowed.into());
        }

        Ok(())
    }

    pub fn assert_can_be_kicker() {
        unimplemented!();
    }

    pub fn assert_can_candidate() {
        unimplemented!();
    }

    // which state of bcToken now?
}

/// get self rating
pub fn get_self_rate_value() -> u64 {
    unimplemented!();
}

/// Checks whether bcToken account exists, is initialized
pub fn assert_is_valid_bc_token(
    program_id: &Pubkey,
    bc_token_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_is_valid_account(bc_token_info, ShihonAccountType::Uninitialized, program_id)
}

/// Deserializes account and checks owner program
pub fn get_bc_token_data(
    program_id: &Pubkey,
    bc_token_info: &AccountInfo,
) -> Result<BcToken, ProgramError> {
    get_account_data::<BcToken>(program_id, bc_token_info)
}

/// Deserializes account and checks the given authority is bcToken's authority
pub fn get_bc_token_data_for_authority(
    program_id: &Pubkey,
    bc_token_info: &AccountInfo,
    bc_token_authority: &Pubkey,
) -> Result<BcToken, ProgramError> {
    let bc_token_data = get_account_data::<BcToken>(program_id, bc_token_info)?;

    if bc_token_data.authority.is_none() {
        return Err(ShihonError::RealmHasNoAuthority.into());
    }

    if bc_token_data.authority.unwrap() != *bc_token_authority {
        return Err(ShihonError::InvalidAuthorityForBcToken.into());
    }

    Ok(bc_token_data)
}

/// Deserializes bcToken account and asserts the given bc_token_mint is token mint of the Tanistry
pub fn get_bc_token_data_for_bc_token_mint(
    program_id: &Pubkey,
    bc_token_info: &AccountInfo,
    bc_token_mint: &Pubkey,
) -> Result<BcToken, ProgramError> {
    let bc_token_data = get_bc_token_data(program_id, bc_token_info)?;

    bc_token_data.assert_is_valid_bc_token_mint(bc_token_mint)?;

    Ok(bc_token_data)
}

/// Returns bcToken PDA seeds
pub fn get_bc_token_address_seeds(name: &str) -> [&[u8]; 2] {
    [PROGRAM_AUTHORITY_SEED, name.as_bytes()]
}

/// Returns bcToken PDA address
pub fn get_bc_token_address(program_id: &Pubkey, name: &str) -> Pubkey {
    Pubkey::find_program_address(&get_bc_token_address_seeds(name), program_id).0
}

/// Returns bcToken Holding PDA seeds
pub fn get_bc_token_holding_address_seeds<'a>(
    bc_token: &'a Pubkey,
    bc_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        bc_token.as_ref(),
        bc_token_mint.as_ref(),
    ]
}

/// Returns bcToken Holding PDA address
pub fn get_bc_token_holding_address(
    program_id: &Pubkey,
    bc_token: &Pubkey,
    bc_token_mint: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_bc_token_holding_address_seeds(bc_token, bc_token_mint),
        program_id,
    )
    .0
}
