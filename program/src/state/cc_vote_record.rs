//! Proposal Vote Record Account

use borsh::maybestd::io::Write;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    clock::{Slot, UnixTimestamp},
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
};
use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::error::ShihonError;

use crate::PROGRAM_AUTHORITY_SEED;

use crate::state::enums::ShihonAccountType;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct CCVoteChoice {
    /// Challenger Ring
    pub challenger_ring: Pubkey,

    /// which ring need to make CC
    pub target_ring: Option<Pubkey>,
}

// impl CCVoteChoice {
//     /// Returns the choice weight given the voter's weight
//     pub fn get_choice_weight(&self, voter_weight: u64) -> Result<u64, ProgramError> {
//         Ok(match self.weight_percentage {
//             100 => voter_weight,
//             0 => 0,
//             _ => return Err(ShihonError::InvalidVoteChoiceWeightPercentage.into()),
//         })
//     }
// }

/// User's vote
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum Vote {
    /// want to pull the other ring to make CC
    Pull(Vec<CCVoteChoice>),

    /// want to push(it means ignored)
    Push,
}

/// CC Vote Record
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct CCVoteRecord {
    /// account type
    pub account_type: ShihonAccountType,

    /// The user who casted this vote
    /// This is the Governing Token Owner who deposited governing tokens into the Realm
    pub governing_token_owner: Pubkey,

    /// when system counted the whole voting
    pub counting_time: Slot,

    /// Indicates whether the vote was relinquished by voter
    pub is_relinquished: bool,

    /// The weight of the user casting the vote
    pub voter_weight: u64,

    /// Voter's vote
    pub vote: Vote,
}

impl AccountMaxSize for CCVoteRecord {}

impl IsInitialized for CCVoteRecord {
    fn is_initialized(&self) -> bool {
        self.account_type == ShihonAccountType::CCVoteRecord
    }
}
impl CCVoteRecord {
    /// Checks the vote can be relinquished
    pub fn assert_can_relinquish_vote(&self) -> Result<(), ProgramError> {
        if self.is_relinquished {
            return Err(ShihonError::VoteAlreadyRelinquished.into());
        }

        Ok(())
    }

    ///TODO: What function should be created to count here?
    /// Serializes account into the target buffer
    pub fn serialize<W: Write>(self, writer: &mut W) -> Result<(), ProgramError> {
        if self.account_type == ShihonAccountType::CCVoteRecord {
            BorshSerialize::serialize(&self, writer)?
        } else if self.account_type == ShihonAccountType::VoteRecordV1 {
            // V1 account can't be resized and we have to translate it back to the original format
            let vote_weight = match &self.vote {
                Vote::Pull(_options) => VoteWeightV1::Yes(self.voter_weight),
                Vote::Push => VoteWeightV1::No(self.voter_weight),
            };

            let vote_record_data_v1 = VoteRecordV1 {
                account_type: self.account_type,
                proposal: self.proposal,
                governing_token_owner: self.governing_token_owner,
                is_relinquished: self.is_relinquished,
                vote_weight,
            };

            BorshSerialize::serialize(&vote_record_data_v1, writer)?;
        }

        Ok(())
    }
}

/// Deserializes CCVoteRecord account data
pub fn get_cc_vote_record_data(
    program_id: &Pubkey,
    vote_record_info: &AccountInfo,
) -> Result<CCVoteRecord, ProgramError> {
    let account_type: ShihonAccountType =
        try_from_slice_unchecked(&vote_record_info.data.borrow())?;

    // If the account is V1 version then translate to V2
    if account_type == ShihonAccountType::CCVoteRecord {
        let vote_record_data_v1 = get_account_data::<CCVoteRecord>(program_id, vote_record_info)?;

        let (vote, voter_weight) = match vote_record_data_v1.vote_weight {
            VoteWeightV1::Yes(weight) => (
                Vote::Pull(vec![CCVoteChoice {
                    rank: 0,
                    weight_percentage: 100,
                }]),
                weight,
            ),
            VoteWeightV1::No(weight) => (Vote::Push, weight),
        };

        return Ok(CCVoteRecord {
            account_type,
            proposal: vote_record_data_v1.proposal,
            governing_token_owner: vote_record_data_v1.governing_token_owner,
            is_relinquished: vote_record_data_v1.is_relinquished,
            voter_weight,
            vote,
        });
    }

    get_account_data::<CCVoteRecord>(program_id, vote_record_info)
}

/// Deserializes CCVoteRecord data for each rings
pub fn get_cc_vote_record_data_for_challenger_ring_and_targeted_ring(
    program_id: &Pubkey,
    vote_record_info: &AccountInfo,
    proposal: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Result<CCVoteRecord, ProgramError> {
    let vote_record_data = get_vote_record_data(program_id, vote_record_info)?;

    if vote_record_data.proposal != *proposal {
        return Err(ShihonError::InvalidProposalForVoterRecord.into());
    }

    if vote_record_data.governing_token_owner != *governing_token_owner {
        return Err(ShihonError::InvalidGoverningTokenOwnerForVoteRecord.into());
    }

    Ok(vote_record_data)
}

/// Returns CCVoteRecord PDA seeds
pub fn get_cc_vote_record_address_seeds<'a>(
    target_ring: &'a Pubkey,
    token_owner_record: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        PROGRAM_AUTHORITY_SEED,
        target_ring.as_ref(),
        token_owner_record.as_ref(),
    ]
}

/// Returns CCVoteRecord PDA address
pub fn get_cc_vote_record_address<'a>(
    program_id: &Pubkey,
    target_ring: &'a Pubkey,
    token_owner_record: &'a Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_cc_vote_record_address_seeds(target_ring, token_owner_record),
        program_id,
    )
    .0
}
