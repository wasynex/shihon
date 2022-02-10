//! Tanistry Account

use crate::{
    error::ShihonError,
    state::enums::{ShihonAccountType, VoteThresholdPercentage, VoteWeightSource},
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

    /// Reserved space for future versions
    pub reserved: [u8; 8],

    /// kicker coin owner record
    pub kicker_coin_owner_record: Pubkey,

    /// CandidateLimitRecord List
    pub candidate_limit_record_list: Vec<Pubkey>,

    /// total amount of coin
    pub total_amount_of_coin: Lamports,
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

/// Checks the given account is a bcToken account and belongs to the given tanistry
pub fn assert_tanistry_for_bc_token(
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
