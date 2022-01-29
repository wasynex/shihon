use crate::error::TokenError::InvalidInstruction;
use solana_program::{
    msg,
    program_error::ProgramError,
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use std::convert::TryInto;

///  ~ Terminate 1
pub enum SetInstruction {
    /// The first kicker decides how much KickerCoin by himself, and then throws it with own bcToken at other's bcToken to initialize (e)RFT as the tanistry ring. Note that all bcTokens here must not have been initialized to (e)RFT at any time.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as first kicker initializing the two bcToken into Tanistry Ring
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the first kicker
    /// 2. `[]` The first kicker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The Tanistry account, it will hold all necessary info about the Tanistry.
    /// 4. `[writable]` The coordinator's token account for input
    /// 5. `[]` The rent sysvar
    /// 6. `[]` The token program
    /// 7. `[]` PDA account
    Kicking {
        /// this para is put as self-rating.
        coordinator: Pubkey,
        amount: u64,
    },
    /// This is a parameter for the candidate. Here, bcToken can be submitted only for (e)RFT to perform the candidate.
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
    Approve {
        /// for making new RFT
        coordinator_input: String,
    },

    Revoke,
}

impl SetInstruction {
    /// Unpacks a byte buffer into a [SetInstruction](enum.SetInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Kicking {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Candidate {
                amount: Self::unpack_amount(rest)?,
            },
            2 => Self::Approve {
                coordinator_input: Self::unpack_coordinator_input()?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < PUBKEY_BYTES {
            msg!("Pubkey cannot be unpacked");
            return Err(InvalidInstruction.into());
        }
        let (key, rest) = input.split_at(PUBKEY_BYTES);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }

    fn unpack_coordinator_input() -> Result<String, ProgramError> {
        unimplemented!();
    }
}

/// Terminate 1 ~ Terminate 2 and Terminate 4 ~ Terminate 5
pub enum RateInstruction {
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
    Mix { link: String },
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
    Rate { rating: u64 },
}

impl RateInstruction {
    pub fn unpack() {
        unimplemented!();
    }
}

/// Terminate 3 ~ Terminate 4
pub enum BumpInstruction {
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate on the Tanistry.
    /// 1. `[writable]` Temporary token account for add self-rating on the same Candidate's content
    /// 2. `[Writable]` The Tanistry account holding the Tanistry info
    /// 3. `[]` The token program
    /// 4. `[]` The PDA account
    Bump { amount: u64 },
}

impl BumpInstruction {
    pub fn unpack() {
        unimplemented!();
    }
}

///Terminate 5 ~ Terminate 6
pub enum SelloutInstruction {
    ///
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
    Sell { amount: u64 },
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
    Buy { amount: u64 },
}

impl SelloutInstruction {
    pub fn unpack() {
        unimplemented!();
    }
}

///Terminate 6 ~
pub enum CrowningInstruction {
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Coordinator's token account for send his input
    /// 2. `[writable]` The Coordinator's token account for receive the NFT as Refund when he drop the game
    /// 3. `[writable]` The new Crown's token account for choosing next coordinator
    /// 4. `[Writable]` The Tanistry account holding the Tanistry info
    /// 5. `[]` The token program
    /// 8. `[]` The PDA account
    Crowning { crown: Pubkey },
}

impl CrowningInstruction {
    pub fn unpack() {
        unimplemented!();
    }
}
