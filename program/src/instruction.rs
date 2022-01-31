use crate::error::ShihonError::InvalidInstruction;
use crate::state::{get_bc_token_address, get_bc_token_holding_address, BcToken};

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[repr(C)]
#[allow(clippy::large_enum_variant)]
pub enum ShihonInstruction {
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate taking the Tanistry.
    /// 1. `[writable]` The Candidate's token account for the token he send as self-rate value
    /// 2. `[writable]` The Candidate's token account for the token they will receive cash as refund when game finished
    /// 3. `[writable]` The kicker's main account to send their rent fees to
    /// 4. `[writable]` The kicker's token account that will receive tokens
    /// 5. `[writable]` The Tanistry account holding the Tanistry info
    /// 6. `[]` The token program
    /// 7. `[]` The PDA account
    CreateBcToken {
        name: String,
        amount: u64,
        // config_args: BcTokenConfigArgs,
    },

    Discard,

    ///  ~ Terminate 1
    /// The first kicker decides how much KickerCoin by himself, and then throws it with own BcToken at other's BcToken to initialize (e)RFT as the tanistry ring. Note that all BcTokens here must not have been initialized to (e)RFT at any time.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as first kicker initializing the two BcToken into Tanistry Ring
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the first kicker
    /// 2. `[]` The first kicker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The Tanistry account, it will hold all necessary info about the Tanistry.
    /// 4. `[writable]` The coordinator's token account for input
    /// 5. `[]` The rent sysvar
    /// 6. `[]` The token program
    /// 7. `[]` PDA account
    /// 8. `[]` System program
    Kicking {
        /// this para is put as self-rating.
        coordinator: Pubkey,
        amount: u64,
    },
    /// This is a parameter for the candidate. Here, BcToken can be submitted only for (e)RFT to perform the candidate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate taking the Tanistry.
    /// 1. `[writable]` The Candidate's token account for the token he send as self-rate value
    /// 2. `[writable]` The Candidate's token account for the token they will receive cash as refund when game finished
    /// 3. `[writable]` The kicker's main account to send their rent fees to
    /// 4. `[writable]` The kicker's token account that will receive tokens
    /// 5. `[writable]` The Tanistry account holding the Tanistry info
    /// 6. `[]` The token program
    /// 7. `[]` The PDA account
    /// 8. `[]` System program
    /// 9. `[]` Sysvar Rent
    Candidate {
        /// for as self-rating
        amount: u64,
    },
    /// for coordinator's choice:
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Coordinator's token account for send his input
    /// 2. `[writable]` The Coordinator's token account fot receive the NFT as Refund when he drop the game
    /// 3. `[writable]` The kicker's main account to send their rent fees to
    /// 4. `[writable]` The kicker's toke account that will receive tokens
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info
    /// 6. `[]` The token program
    /// 7. `[]` The PDA account
    /// 8. `[]` System program
    /// 9. `[]` Sysvar Rent
    Approve {
        /// for making new RFT
        coordinator_input: String,
    },

    Deny,

    ////// Terminate 1 ~ Terminate 2 and Terminate 4 ~ Terminate 5
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Rater's token account for send his input the rating amount
    /// 2. `[writable]` The Buddy Candidate's token account for mix the content
    /// 3. `[writable]` The first kicker's (Initial content) token account for mix the content
    /// 4. `[writable]` The coordinator's input token account for minting RFT for the rating
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info
    /// 6. `[]` The rating program
    /// 7. `[]` The PDA account
    /// 8. `[]` System program
    /// 9. `[]` Sysvar Rent
    Mix {
        link: String,
    },
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Rater's token account for send his input the rating amount
    /// 2. `[writable]` The Buddy Candidate's token account for mix the content
    /// 3. `[writable]` The first kicker's (Initial content) token account for mix the content
    /// 4. `[writable]` The coordinator's input token account for minting RFT for the rating
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info
    /// 6. `[]` The rating program
    /// 7. `[]` The PDA account
    /// 8. `[]` System program
    /// 9. `[]` Sysvar Rent
    Rate {
        rating: u64,
    },

    ////// Terminate 3 ~ Terminate 4
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate on the Tanistry.
    /// 1. `[writable]` Temporary token account for add self-rating on the same Candidate's content
    /// 2. `[Writable]` The Tanistry account holding the Tanistry info
    /// 3. `[]` The token program
    /// 4. `[]` The PDA account
    /// 5. `[]` System program
    /// 6. `[]` Sysvar Rent
    Bump {
        amount: u64,
    },

    //////Terminate 5 ~ Terminate 6
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate on the Tanistry.
    /// 1. `[writable]` The Candidate's token account for selling own token as NFT
    /// 2. `[writable]` Temporary token account for selling NFT of Candidate
    /// 3. `[writable]` The Buyer's token account to buy NFT of Candidate
    /// 4. `[]` The Buyer's token account for the token they will receive refund if won the game
    /// 6. `[Writable]` The Tanistry account holding the Tanistry info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    /// 9. `[]` System program
    /// 10. `[]` Sysvar Rent
    Sell {
        amount: u64,
    },
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Buyer on the Tanistry.
    /// 1. `[writable]` The Candidate's token account for selling own token as NFT
    /// 2. `[writable]` Temporary token account for selling NFT of Candidate
    /// 3. `[writable]` The Buyer's token account to buy NFT of Candidate
    /// 4. `[]` The Buyer's token account for the token they will receive refund if won the game
    /// 6. `[Writable]` The Tanistry account holding the Tanistry info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    /// 9. `[]` System program
    /// 10. `[]` Sysvar Rent
    Buy {
        amount: u64,
    },

    ///Terminate 6 ~
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Coordinator's token account for send his input
    /// 2. `[writable]` The Coordinator's token account for receive the NFT as Refund when he drop the game
    /// 3. `[writable]` The new Crown's token account for choosing next coordinator
    /// 4. `[Writable]` The Tanistry account holding the Tanistry info
    /// 5. `[]` The token program
    /// 6. `[]` The PDA account
    /// 7. `[]` System program
    /// 8. `[]` Sysvar Rent
    Crowning {
        crown: Pubkey,
    },
}

/// Create bcToken instruction
pub fn create_bc_token(
    program_id: &Pubkey,
    // Accounts
    bc_token_authority: &Pubkey,
    bc_token_mint: &Pubkey,
    payer: &Pubkey,
    // Args
    name: String,
    amount: u64,
) -> Instruction {
    let bc_token_address = get_bc_token_address(program_id, &name);
    let bc_token_holding_address =
        get_bc_token_holding_address(program_id, &bc_token_address, bc_token_mint);

    let mut accounts = vec![
        AccountMeta::new(bc_token_address, false),
        AccountMeta::new_readonly(*bc_token_authority, false),
        AccountMeta::new_readonly(*bc_token_mint, false),
        AccountMeta::new(bc_token_holding_address, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let instruction = ShihonInstruction::CreateBcToken { name, amount };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
/// for kicking to another bcToken
pub fn kicking(
    program_id: &Pubkey,
    // Accounts
    kicker_token_mint: &Pubkey,
    bc_token_authority: &Pubkey,
    payer: &Pubkey,
    // Args
    coordinator: &Pubkey,
    amount: u64,
) -> Instruction {
    let kicker_token_address = get_kicker_token_address(program_id);
    let kicker_token_holding_address =
        get_kicker_token_holding_address(program_id, &kicker_token_address, kicker_token_mint);
    let mut accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let instruction = ShihonInstruction::Kicking {
        coordinator,
        amount,
    };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// for candidate to (e)RFT after accepted KickerCoin
pub fn candidate() {
    unimplemented!();
}
/// for Coordinator' choice
pub fn approve_kicker_coin() {
    unimplemented!();
}
/// for Coordinator's choice
pub fn revoke_kicker_coin() {
    unimplemented!();
}

pub fn mix_content_for_rating() {
    unimplemented!();
}

pub fn rate_content() {
    unimplemented!();
}

pub fn bump_self_rate() {
    unimplemented!();
}

pub fn sellout_extra_rating() {
    unimplemented!();
}

pub fn buy_content_token() {
    unimplemented!();
}

pub fn crowning() {
    unimplemented!();
}

pub fn kicking_to_next_coordinator() {
    unimplemented!();
}
