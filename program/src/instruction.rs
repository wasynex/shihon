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
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    /// 6. `[]` The rate program
    InitTanistry {
        /// this para is put as self-rating.
        coordinator: Pubkey,
        amount: u64,
    },
    /// This is a parameter for the candidate. Here, bcToken can be submitted only for (e)RFT to perform the candidate.
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Candidate taking the Tanistry.
    /// 1. `[writable]` The taker's token account for the token they send
    /// 2. `[writable]` The taker's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The PDA's temp token account to get tokens from and eventually close
    /// 4. `[writable]` The initializer's main account to send their rent fees to
    /// 5. `[writable]` The initializer's token account that will receive tokens
    /// 6. `[writable]` The Tanistry account holding the Tanistry info
    /// 7. `[]` The token program
    /// 8. `[]` The PDA account
    Candidate {
        /// for as self-rating
        amount: u64,
    },
    /// for coordinator's choice
    Accept {
        /// for making new RFT
        coordinator_input: String,
    },
    Deny {
        deny_input: String,
    },
}

impl SetInstruction {
    /// Unpacks a byte buffer into a [SetInstruction](enum.SetInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitTanistry {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Candidate {
                amount: Self::unpack_amount(rest)?,
            },
            2 => Self::Accept {
                coordinator_input: Self::unpack_coordinator_input()?,
            },
            3 => Self::Deny {
                deny_input: Self::unpack_deny_input()?,
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

    fn unpack_deny_input() -> Result<String, ProgramError> {
        unimplemented!();
    }
}

/// Terminate 1 ~ Terminate 2
/// Terminate 4 ~ Terminate 5
pub enum RateInstruction {
    Mix { link: String },
    Rate { rating: u64 },
}

impl RateInstruction {}

/// Terminate 3 ~ Terminate 4
pub enum BumpInstruction {}

impl BumpInstruction {}

///Terminate 5 ~ Terminate 6
pub enum SelloutInstruction {}

impl SelloutInstruction {}

///Terminate 6 ~
pub enum CrowningInstruction {}

impl CrowningInstruction {}
