//! State enumerations

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Shihon accounts types
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ShihonAccountType {
    /// Default uninitialized account state
    Uninitialized,

    /// bcToken
    BcToken,

    /// bcToken's inside metadata
    BcTokenMetadata,

    /// KickerCoin Owner Record for kicker and coordinator before forming a Tanistry
    KickerCoinOwnerRecord,

    /// Tanistry
    Tanistry,

    /// for candidate limit
    CandidateLimitRecord,

    /// Mix Content Record
    MixContentRecord,

    /// Rate Option for holding some accounts
    RateOption,

    /// for rating other's content
    RateOtherRecord,

    /// Outside buyer account
    OutsideBuyerRecord,

    /// for cc voting
    CCVoteRecord,
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
    /// bcToken before content itself reaches to Oracle
    DraftBlankCheck,

    /// some error happened when created
    ExecutingWithErrors,

    /// just for holding own content in his wallet when its content has reached to Oracle
    HoldingOnPrivate,

    /// for casting own content on public for waiting a KickerCoin by kicker
    PublicOnTheGround,

    /// enable to candidate on Tanistry because coordinator has approved KickerCoin
    EnableToCandidate,

    /// Deadline has come for candidates
    TanistrySetIn,

    /// enable to vote for CC because minimum number of round has done
    EnableToVoteToCC,

    /// Enable to refund because everything has finished
    EnableToRefund,

    /// already been refunded by someone
    AlreadyRefunded,

    /// when holder has cancelled to turn it to private
    Cancelled,
}

impl Default for BcTokenState {
    fn default() -> Self {
        BcTokenState::DraftBlankCheck
    }
}

/// The source of vote for CC
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum VoteSource {
    // Which Ring you Vote
    Transit,

    ///Pull type: Which push or pull
    Snapshot,
}
