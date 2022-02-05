use solana_program::clock::UnixTimestamp;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{assert_is_valid_account, get_account_data, AccountMaxSize};

use crate::{error::ShihonError, state::enums::ShihonAccountType, PROGRAM_AUTHORITY_SEED};

/// bcToken Account
/// Account PDA seeds" ['shihon', name]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct BcToken {

    pub is_initialized: bool,
    pub issuer_pubkey: Pubkey,
    pub number_of_issue: u64,
    pub link_of_content: String,
    pub issue_at: UnixTimestamp,
    /// Governance account type
    pub account_type: ShihonAccountType,

    /// Community mint
    pub community_mint: Pubkey,

    /// Configuration of the Realm
    pub config: BcTokenConfig,

    /// Reserved space for future versions
    pub reserved: [u8; 8],

    /// Realm authority. The authority must sign transactions which update the realm config
    /// The authority should be transferred to Realm Governance to make the Realm self governed through proposals
    pub authority: Option<Pubkey>,

    /// Governance Realm name
    pub name: String,

    pub mpc_key: Option<u8>,

}

impl IsInitialized for BcToken {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
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


impl AccountMaxSize for BcToken {
    fn get_max_size(&self) -> Option<usize> {
        Some(self.name.len() + 136)
    }
}

impl IsInitialized for BcToken {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::
    }
}

impl BcToken {
    /// Asserts the given mint is either Community or Council mint of the Realm
    pub fn assert_is_valid_governing_token_mint(
        &self,
        governing_token_mint: &Pubkey,
    ) -> Result<(), ProgramError> {
        if self.community_mint == *governing_token_mint {
            return Ok(());
        }

        if self.config.council_mint == Some(*governing_token_mint) {
            return Ok(());
        }

        Err(ShihonError::InvalidGoverningTokenMint.into())
    }

    /// Asserts the given governing token mint and holding accounts are valid for the realm
    pub fn assert_is_valid_governing_token_mint_and_holding(
        &self,
        program_id: &Pubkey,
        realm: &Pubkey,
        governing_token_mint: &Pubkey,
        governing_token_holding: &Pubkey,
    ) -> Result<(), ProgramError> {
        self.assert_is_valid_governing_token_mint(governing_token_mint)?;

        let governing_token_holding_address =
            get_governing_token_holding_address(program_id, realm, governing_token_mint);

        if governing_token_holding_address != *governing_token_holding {
            return Err(ShihonError::InvalidGoverningTokenHoldingAccount.into());
        }

        Ok(())
    }

    /// Asserts the given governing token can be deposited into the realm
    pub fn asset_governing_tokens_deposits_allowed(
        &self,
        governing_token_mint: &Pubkey,
    ) -> Result<(), ProgramError> {
        // If the deposit is for the community token and the realm uses community voter weight addin then panic
        if self.config.use_community_voter_weight_addin
            && self.community_mint == *governing_token_mint
        {
            return Err(ShihonError::GoverningTokenDepositsNotAllowed.into());
        }

        Ok(())
    }
}

/// Checks whether realm account exists, is initialized and  owned by Governance program
pub fn assert_is_valid_realm(
    program_id: &Pubkey,
    realm_info: &AccountInfo,
) -> Result<(), ProgramError> {
    assert_is_valid_account(realm_info, ShihonAccountType::Realm, program_id)
}

/// Deserializes account and checks owner program
pub fn get_realm_data(
    program_id: &Pubkey,
    realm_info: &AccountInfo,
) -> Result<Realm, ProgramError> {
    get_account_data::<Realm>(program_id, realm_info)
}

/// Deserializes account and checks the given authority is Realm's authority
pub fn get_realm_data_for_authority(
    program_id: &Pubkey,
    realm_info: &AccountInfo,
    realm_authority: &Pubkey,
) -> Result<Realm, ProgramError> {
    let realm_data = get_account_data::<Realm>(program_id, realm_info)?;

    if realm_data.authority.is_none() {
        return Err(ShihonError::RealmHasNoAuthority.into());
    }

    if realm_data.authority.unwrap() != *realm_authority {
        return Err(ShihonError::InvalidAuthorityForRealm.into());
    }

    Ok(realm_data)
}

/// Deserializes Ream account and asserts the given governing_token_mint is either Community or Council mint of the Realm
pub fn get_realm_data_for_governing_token_mint(
    program_id: &Pubkey,
    realm_info: &AccountInfo,
    governing_token_mint: &Pubkey,
) -> Result<Realm, ProgramError> {
    let realm_data = get_realm_data(program_id, realm_info)?;

    realm_data.assert_is_valid_governing_token_mint(governing_token_mint)?;

    Ok(realm_data)
}

/// Returns Realm PDA seeds
pub fn get_realm_address_seeds(name: &str) -> [&[u8]; 2] {
    [PROGRAM_AUTHORITY_SEED, name.as_bytes()]
}

/// Returns Realm PDA address
pub fn get_realm_address(program_id: &Pubkey, name: &str) -> Pubkey {
    Pubkey::find_program_address(&get_realm_address_seeds(name), program_id).0
}

/// Returns Realm Token Holding PDA seeds
pub fn get_governing_token_holding_address_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        realm.as_ref(),
        governing_token_mint.as_ref(),
    ]
}

/// Returns Realm Token Holding PDA address
pub fn get_governing_token_holding_address(
    program_id: &Pubkey,
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_governing_token_holding_address_seeds(realm, governing_token_mint),
        program_id,
    )
    .0
}

/// Asserts given realm config args are correct
pub fn assert_valid_realm_config_args(config_args: &BcTokenConfigArgs) -> Result<(), ProgramError> {
    match config_args.community_mint_max_vote_weight_source {
        MintMaxVoteWeightSource::SupplyFraction(fraction) => {
            if !(1..=MintMaxVoteWeightSource::SUPPLY_FRACTION_BASE).contains(&fraction) {
                return Err(ShihonError::InvalidMaxVoteWeightSupplyFraction.into());
            }
        }
        MintMaxVoteWeightSource::Absolute(_) => {
            return Err(ShihonError::MintMaxVoteWeightSourceNotSupported.into())
        }
    }

    Ok(())
}

