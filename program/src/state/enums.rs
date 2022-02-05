//! State enumerations

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Shihon accounts types
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ShihonAccountType {
    /// Default uninitialized account state
    Uninitialized,

    /// Token Owner Record for given governing token owner within a Tanistry
    TokenOwnerRecord,

    /// for candidate limit
    CandidateLimitRecord,

    /// for rating other's content
    RateOtherRecord,

    /// Outside buyer account
    OutsideBuyerRecord,

    /// for cc voting
    CCVoteRecord,

    /// bcToken's inside metadata
    BcTokenMetadata,
}

impl Default for ShihonAccountType {
    fn default() -> Self {
        ShihonAccountType::Uninitialized
    }
}

/// What state a Proposal is in
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum BcTokenState {
    /// before creating bcToken
    Draft,

    /// just for holding own content when reached to Oracle
    Private,

    /// for casting own content for waiting a KickerCoin
    Public,

    /// enable to candidate on Tanistry
    Executing,

    /// Voting ended with success
    Succeeded,

    /// Enable to refund
    Completed,

    /// Cancelled
    Cancelled,

    /// some error happened when created
    ExecutingWithErrors,

    /// Taking votes
    Voting,
}

impl Default for BcTokenState {
    fn default() -> Self {
        BcTokenState::Draft
    }
}
