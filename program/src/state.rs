use solana_program::clock::{Slot, UnixTimestamp};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct BCToken {
    pub is_initialized: bool,
    pub number_of_issue: u64,
    pub link_of_content: String,
    pub issue_at: UnixTimestamp,
    pub issuer_pubkey: Pubkey,
}

pub struct Tanistry {
    pub is_initialized: bool,
    pub kicker_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub kicker_token_to_receive_account_pubkey: Pubkey,
    // pub next_tanistry_id: Pubkey;
    // pub previous_tanistry_id: Pubkey;
    // pub rating_list: [Rate],
    // pub number_of_round: u64,
    // pub mpc_key: Vec<u8>
    // pub building_key: Vec<u8>,
    // pub iterator_of_forward_for_minting: u64,
}

impl Sealed for Tanistry {}

impl IsInitialized for Tanistry {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Tanistry {
    const LEN: usize = 105;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Tanistry::LEN];
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Tanistry {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initializer_token_to_receive_account_pubkey,
            ),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Tanistry::LEN];
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Tanistry {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst
            .copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }
}

/// state for rating the content
pub struct Rate {
    pub is_initialized: bool,
    pub rated_content: Pubkey,
    pub rater_pubkey: Pubkey,
    pub rate_amount: u64,
    pub temp_rating_account_pubkey: Pubkey,
}

impl Sealed for Rate {}

impl IsInitialized for Rate {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Rate {
    const LEN: usize = 96;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        unimplemented!();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        unimplemented!();
    }
}

/// for voting ring
pub struct Vote {
    pub is_initialized: bool,
    pub is_pulling: bool,
    pub targeted_ring: Pubkey,
}

impl Sealed for Vote {}

impl IsInitialized for Vote {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Vote {
    const LEN: usize = 96;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        unimplemented!();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        unimplemented!();
    }
}
