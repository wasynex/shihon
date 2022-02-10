use solana_program::clock::UnixTimestamp;

use crate::state::bctoken_metadata::{BcTokenConfig, BcTokenConfigArgs};
use crate::state::enums::BcTokenState;
use crate::{error::ShihonError, state::enums::ShihonAccountType, PROGRAM_AUTHORITY_SEED};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{assert_is_valid_account, get_account_data, AccountMaxSize};

/// bcToken Account
/// Account PDA seeds" ['shihon', name]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcToken {
    pub is_initialized: bool,
    /// account type
    pub account_type: ShihonAccountType,
    pub issuer_pubkey: Pubkey,
    pub amount_of_coin: Lamports,
    pub number_of_issue: u64,
    pub link_of_content: String,
    pub issue_at: UnixTimestamp,

    /// Configuration of bcToken
    pub config: BcTokenConfig,

    /// Reserved space for future versions
    pub reserved: [u8; 8],

    /// bcToken authority
    pub authority: Option<Pubkey>,

    /// bcToken name
    pub name: String,

    /// state bcToken
    pub bc_token_state: BcTokenState,

    /// If candidate has this hash, we call him "Roydamna"
    pub mpc_key: Option<u8>,
}

impl IsInitialized for BcToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
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
        if self.config.bc_token_mint == *bc_token_mint {
            return Ok(());
        }
        Err(ShihonError::InvalidGoverningTokenMint.into())
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
            return Err(ShihonError::InvalidGoverningTokenHoldingAccount.into());
        }

        Ok(())
    }

    /// Asserts the given governing token can be deposited into the realm
    pub fn asset_governing_tokens_deposits_allowed(
        &self,
        bc_token_mint: &Pubkey,
    ) -> Result<(), ProgramError> {
        // If the deposit is for the community token and the realm uses community voter weight addin then panic
        if self.config.use_community_voter_weight_addin
            && self.config.bc_token_mint == *bc_token_mint
        {
            return Err(ShihonError::GoverningTokenDepositsNotAllowed.into());
        }

        Ok(())
    }
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
