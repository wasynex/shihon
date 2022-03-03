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
    /// account type
    pub account_type: ShihonAccountType,

    /// next tanistry id
    pub next_tanistry_id: Option<Pubkey>,

    /// previous tanistry id
    pub previous_tanistry_id: Option<Pubkey>,

    /// Reserved space for future versions
    pub reserved: [u8; 8],

    /// kicker coin owner record
    pub kicker_coin_owner_record: Pubkey,

    /// CandidateLimitRecord List
    pub candidate_limit_record_list: Vec<Pubkey>,
}

impl AccountMaxSize for Tanistry {}

impl IsInitialized for Tanistry {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::Tanistry
    }
}

// Tanistry itself doesn't have process function, but has some works
// 1. role as a vast for candidate's payment
// 2. to shuffle the candidate list for rating
// 3. for circuit the MPC key to distributing to all candidates

impl Tanistry {
    /// Returns Tanistry PDA seeds (All in one)
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

    /// make the mpc hash key for identity of Roydamna inside the Tanistry
    fn make_mpc_key(first_candidate: Pubkey, second_candidate: Pubkey) -> Pubkey {
        unimplemented!();
        // all hash for identity like this MPC Key is on each CLR(CandidateLimitRecord), not bcToken itself
        //
    }

    /// make (pair of candidate + first kicker) triple person
    fn make_triple(candidate_list: Vec<Pubkey>) {
        unimplemented!();
        // The coordinator is not directly involved in the relationship of this triangle
        // his work is only to give his input and proof of his existence to the first kicker(aka init content holder).
    }

    /// shuffle the candidate
    fn shuffle_candidate() {
        unimplemented!();
    }

    /// getting the triple person for rating
    fn get_triple(candidate: Pubkey) {
        unimplemented!();
    }

    fn get_my_buddy_candidate(me: Pubkey) -> Pubkey {
        unimplemented!();
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

/// Returns Mint Tanistry PDA seeds
pub fn get_mint_tanistry_address_seeds<'a>(
    bc_token: &'a Pubkey,
    tanistry_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    // 'mint-governance' prefix ensures uniqueness of the PDA
    // Note: Only the current mint authority can create an account with this PDA using CreateMintGovernance instruction
    [b"mint-tanistry", bc_token.as_ref(), tanistry_mint.as_ref()]
}

/// Returns Mint Tanistry PDA address
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

/// Returns Token Tanistry PDA seeds
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

/// Returns Token Tanistry PDA address
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

/// Returns Account Tanistry PDA seeds
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

/// these funcs moved from modules/value.rs
/// sum of members of one tanistry
fn get_tanistry_value() -> u64 {
    unimplemented!();
}
/// sum of 1,3,5 of tanistry value
fn get_building_value() -> u64 {
    unimplemented!();
}
/// sum of both wallet (α + β)
fn get_ring_value() -> u64 {
    unimplemented!();
}

fn get_cc_value() -> u64 {
    unimplemented!();
}
