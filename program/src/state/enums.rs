//! State enumerations

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

/// Defines all Shihon accounts types
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub enum ShihonAccountType {
    /// Default uninitialized account state
    Uninitialized,

    /// Top level aggregation for governances with Community Token (and optional Council Token)
    Realm,

    /// Token Owner Record for given governing token owner within a Realm
    TokenOwnerRecord,

    /// Generic Account Governance account
    AccountGovernance,

    /// Program Governance account
    ProgramGovernance,

    /// Proposal account for Governance account. A single Governance account can have multiple Proposal accounts
    ProposalV1,

    /// Proposal Signatory account
    SignatoryRecord,

    /// Vote record account for a given Proposal.  Proposal can have 0..n voting records
    VoteRecordV1,

    /// ProposalInstruction account which holds an instruction to execute for Proposal
    ProposalInstructionV1,

    /// Mint Governance account
    MintGovernance,

    /// Token Governance account
    TokenGovernance,

    /// Realm config account
    RealmConfig,

    /// Vote record account for a given Proposal.  Proposal can have 0..n voting records
    /// V2 adds support for multi option votes
    VoteRecordV2,

    /// ProposalInstruction account which holds an instruction to execute for Proposal
    /// V2 adds index for proposal option
    ProposalInstructionV2,

    /// Proposal account for Governance account. A single Governance account can have multiple Proposal accounts
    /// V2 adds support for multiple vote options
    ProposalV2,

    /// Program metadata account. It stores information about the particular SPL-Governance program instance
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
    /// Draft - Proposal enters Draft state when it's created
    Draft,

    /// SigningOff - The Proposal is being signed off by Signatories
    /// Proposal enters the state when first Signatory Sings and leaves it when last Signatory signs
    SigningOff,

    /// Taking votes
    Voting,

    /// Voting ended with success
    Succeeded,

    /// Voting on Proposal succeeded and now instructions are being executed
    /// Proposal enter this state when first instruction is executed and leaves when the last instruction is executed
    Executing,

    /// Completed
    Completed,

    /// Cancelled
    Cancelled,

    /// Defeated
    Defeated,

    /// Same as Executing but indicates some instructions failed to execute
    /// Proposal can't be transitioned from ExecutingWithErrors to Completed state
    ExecutingWithErrors,
}

impl Default for BcTokenState {
    fn default() -> Self {
        BcTokenState::Draft
    }
}
