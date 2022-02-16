use crate::{
    state::{
        bc_token::{get_bc_token_address, get_bc_token_holding_address, BcToken},
        bc_token_metadata::BcTokenMetadata,
        kicker_coin_owner_record::get_kicker_coin_owner_record_address,
    },
    tools::bpf_loader_upgradeable::get_program_data_address,
};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    bpf_loader_upgradeable,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
#[repr(C)]
#[allow(clippy::large_enum_variant)]
pub enum ShihonInstruction {
    ///
    /// Accounts expected:
    ///
    /// 0. `[writable]` bcToken account. PDA seeds:['bcToken',name]
    /// 1. `[]` bcToken authority
    /// 2. `[]` KickerCoin's Token Mint
    /// 3. `[writable]` KickerCoin's Token Holding account. PDA seeds: ['bcTOken',bcToken,kicker_coin_mint]
    ///     The account will be created with the bcToken PDA as its owner
    /// 4. `[signer]` The account of the person as Candidate taking the Tanistry.
    /// 5. `[writable]` Candidate's self-rating Coin Token Holding account PDA seeds: ['bcToken',bcToken,candidate_mint]
    /// 6. `[writable]` BcTokenMetadata account. PDA seeds: ['bcToken-metadata', bcToken]
    /// 7. `[]` System
    /// 8. `[]` SPL Token
    /// 9. `[]` Sysvar Rent
    /// 10. `[]` Candidate's self-rating Coin Token Mint
    CreateBcToken {
        name: String,
        amount: u64,
        config: BcTokenMetadata,
    },

    DiscardBcToken,

    ///  ~ Terminate 1
    /// The first kicker decides how much KickerCoin by himself, and then throws it with own BcToken at other's BcToken to initialize (e)RFT as the tanistry ring. Note that all BcTokens here must not have been initialized to (e)RFT at any time.
    ///
    ///
    /// Accounts expected:
    ///
    ///   0. `[writable]` Proposal account
    ///   1. `[]` TokenOwnerRecord account of the Proposal owner
    ///   2. `[signer]` The account of the person as first kicker initializing the two BcToken into Tanistry Ring
    ///   3. `[writable]` KickerCoin Owner Record
    ///   4. `[writable]` Coordinator's bcToken account to flag as received KickerCoin from first kicker
    ///   5. `[]` Clock sysvar
    Kicking {
        /// this para is put as self-rating.
        coordinator: Pubkey,
        amount: u64,
    },

    /// for coordinator's choice:
    ///
    /// Accounts expected:
    ///
    ///
    /// 0. `[]` Tanistry account the created bcToken belongs to
    /// 1. `[signer]` Coordinator's account for starting the Tanistry.(Payer)
    /// 2. `[Writable]` The Tanistry account holding the Tanistry info. PDA seeds: ['tanistry', bcToken, tanistry_account]
    /// 3. `[writable]` The Coordinator's token account for send his input PDA seeds: ['account-tanistry', bcToken, tanistry_account]
    /// 4. `[writable]` The Kicker's token Source account for the token he send his KickerCoin into Coordinator's wallet.
    /// 5. `[signer]` Tanistry authority (Coordinator has this!)
    /// 6. `[writable]` The Coordinator's token account fot receive the NFT as Refund when he drop the game
    /// 7. `[writable]` Tanistry TokenOwnerRecord account
    /// 8. `[writable]` The kicker's token account that will receive tokens
    /// 9. `[writable]` Candidate's self-rating Coin Token Holding account PDA seeds: ['bcToken',bcToken,candidate_mint]
    /// 11. `[]` The PDA account
    /// 12. `[]` System program
    /// 13. `[]` Sysvar Rent
    /// 14. `[]` Clock sysvar
    Approve {
        /// for making new RFT
        coordinator_input: String,
    },
    /// Cancels KickerCoin by first kicker for candidate before Tanistry
    ///
    /// Accounts expected:
    ///
    ///   0. `[]` Tanistry account the created bcToken belongs to
    ///   1. `[]` Tanistry TokenOwnerRecord account
    ///   2. `[signer]` Tanistry authority (Coordinator has this!)
    ///   3. `[writable]` KickerCoin Owner Record
    ///   4. `[]` Clock sysvar
    Deny,

    /// This is a parameter for the candidate. Here, BcToken can be submitted only for (e)RFT to perform the candidate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[]` Tanistry account he created bcToken belongs to
    /// 1. `[signer]` Candidate's account for becoming Roydamna in the Tanistry.
    /// 2. `[writable]` The Candidate's token Source account for the token he send as self-rate value. All tokens from the account will be transferred to the Holding account.
    /// 3. `[writable]` Tanistry Token Holding account. PDA seeds: ['tanistry',bcToken, tanistry_token_mint]
    /// 4. `[]` The Candidate's token account for the token they will receive cash as refund when game finished
    /// 6. `[signer]` The kicker's token account that will receive tokens.
    /// 7. `[writable]` Tanistry TokenOwnerRecord account
    /// 8. `[writable]` The Tanistry account holding the Tanistry info PDA seeds: ['tanistry', bcToken, tanistry_token_mint]
    /// 9. `[]` The SPL Token program
    /// 10. `[]` The PDA account
    /// 11. `[]` System program
    /// 12. `[]` Sysvar Rent
    Candidate {
        coordinator: Pubkey,
        amount: u64,
    },
    ////// Terminate 1 ~ Terminate 2 and Terminate 4 ~ Terminate 5
    ///
    /// Accounts expected:
    ///
    ///
    /// 0. `[]` Coordinator's account
    /// 1. `[writable]` The Rater Candidate's token account for send his input the rating amount
    /// 2. `[writable]` The Buddy Candidate's token account for mix the own content
    /// 3. `[writable]` The first kicker's (Initial content) account for mix the content
    /// 4. `[]` The coordinator's input token account for minting RFT for the rating
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: ['shihon', mix, index]
    /// 6. `[signer]` Content Authority (Rater or Buddy)
    /// 7. `[signer]` Buddy Candidate's account
    /// 8. `[writable]` MixContentRecord account
    /// 9. `[]` The rating program
    /// 10. `[]` System program
    /// 11. `[]` Sysvar Rent
    Mix {
        time_shift_a: u64,
        time_shift_b: u64,
    },
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` Rater Candidate account
    /// 1. `[writable]` The Rater Candidate account for send his input the rating amount
    /// 2. `[writable]` The Buddy Candidate's token account for mix the content
    /// 3. `[writable]` The first kicker's (Initial content) account for mix the content
    /// 4. `[]` The coordinator's input token account for minting RFT for the rating
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: ['shihon', mix, index]
    /// 6. `[writable]` MixContentRecord account
    /// 7. `[]` The rating program
    /// 8. `[]` System program
    /// 9. `[]` Sysvar Rent
    Rate {
        rating: u64,
    },

    ////// Terminate 3 ~ Terminate 4
    ///
    /// Accounts expected:
    ///
    ///
    /// 0. `[signer]` The Candidate authority of bcToken
    /// 1. `[signer]` bcToken Authority
    /// 2. `[writable]` The token source of same content holder as candidate
    /// 3. `[writable]` Temporary token account for add self-rating on the same Candidate's content PDA seeds: []
    /// 4. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
    /// 5. `[writable]` Tanistry TokenOwnerRecord account
    /// 6. `[]` The SPL Token program
    /// 7. `[]` System program
    /// 8. `[]` Sysvar Rent
    Bump {
        amount: u64,
    },

    //////Terminate 5 ~ Terminate 6
    ///
    /// Accounts expected:
    ///
    ///
    /// 0. `[signer]` Inside Seller account
    /// 1. `[writable]` TokenSelloutRecord account. PDA seeds: ['tanistry',bcToken, tanistry_token_mint, tanistry_token_owner]
    /// 2. `[writable]` The Seller's token account for selling own token as NFT
    /// 3. `[writable]` The Buyer's token account to buy NFT of Candidate
    /// 4. `[]` The Buyer's token account for the token they will receive refund if won the game
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
    /// 6. `[signer]` bcToken authority
    /// 7. `[]` SPL Token program
    /// 8. `[]` System program
    /// 9. `[]` Clock sysvar
    /// 10. `[]` Sysvar Rent
    Sell,
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` Outside Buyer account
    /// 1. `[writable]` TokenSelloutRecord account. PDA seeds: ['tanistry',bcToken, tanistry_token_mint, tanistry_token_owner]
    /// 2. `[writable]` The Seller's token account for selling own token as NFT
    /// 3. `[writable]` The Buyer's token account to buy NFT of Candidate
    /// 4. `[]` The Buyer's token account for the token they will receive refund if won the game
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
    /// 6. `[]` The seller token Mint
    /// 7. `[]` System program
    /// 8. `[]` Clock sysvar
    /// 9. `[]` Sysvar Rent
    Buy {
        token: Pubkey,
        amount: u64,
    },

    ///Terminate 6 ~
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person as Coordinator taking the Tanistry.
    /// 1. `[writable]` The Coordinator's token account for send his input
    /// 2. `[writable]` The Coordinator's token account for receive the NFT as Refund when he drop the game
    /// 3. `[writable]` KickerCoin Owner Record(from Coordinator to Candidate as next Crown)
    /// 4. `[writable]` The new Crown's token account for choosing next coordinator
    /// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
    /// 6. `[]` The token program
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
    config: BcTokenMetadata,
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

    let instruction = ShihonInstruction::CreateBcToken {
        name,
        amount,
        config,
    };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// kicking KickerCoin to another bcToken for making (e)RFT instruction
pub fn kicking_coin(
    program_id: &Pubkey,
    // Accounts
    kicker_token_mint: &Pubkey,
    kicker_coin_owner_record: &Pubkey,
    bc_token_authority: &Pubkey,
    payer: &Pubkey,
    // Args
    coordinator: &Pubkey,
    amount: u64,
) -> Instruction {
    let kicker_coin_owner_record_address =
        get_kicker_coin_owner_record_address(program_id, kicker_token_mint, coordinator);

    let accounts = vec![
        AccountMeta::new(*kicker_token_mint, false),
        AccountMeta::new_readonly(*kicker_coin_owner_record, false),
        AccountMeta::new_readonly(*bc_token_authority, true),
        AccountMeta::new(kicker_coin_owner_record_address, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let instruction = ShihonInstruction::Kicking {
        coordinator: *coordinator,
        amount,
    };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Instruction whether the coordinator approve the KickerCoin
#[allow(clippy::too_many_arguments)]
pub fn approve_kicker_coin(
    program_id: &Pubkey,
    // Accounts
    tanistry_account: &Pubkey,
    tanistry_holding: &Pubkey,
    tanistry_authority: &Pubkey,
    tanistry_token_mint: &Pubkey,
    payer: &Pubkey,
    tanistry_index: &Pubkey,
    // Args
    coordinator_input: String,
) -> Instruction {
    let tanistry_address = get_tanistry_address(
        program_id,
        tanistry_holding,
        tanistry_token_mint,
        &tanistry_index.to_le_bytes(),
    );

    let mut accounts = vec![
        AccountMeta::new_readonly(*tanistry_account, false),
        AccountMeta::new(tanistry_index, false),
        AccountMeta::new(*tanistry_holding, false),
        AccountMeta::new(*tanistry_authority, false),
        AccountMeta::new_readonly(*tanistry_token_mint, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];

    let instruction = ShihonInstruction::Approve { coordinator_input };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Instruction whether the coordinator approve the KickerCoin
pub fn deny_kicker_coin(
    program_id: &Pubkey,
    // Accounts
    tanistry_account: &Pubkey,
    tanistry_token_owner_record: &Pubkey,
    bc_token_authority: &Pubkey,
    payer: &Pubkey,
    kicker_coin_owner_record: &Pubkey,
) -> Instruction {
    let kicker_coin_owner_record_address =
        get_kicker_coin_owner_record_address(program_id, bc_token_authority, payer);

    let accounts = vec![
        AccountMeta::new_readonly(*tanistry_account, false),
        AccountMeta::new_readonly(*tanistry_token_owner_record, false),
        AccountMeta::new_readonly(*bc_token_authority, true),
        AccountMeta::new(kicker_coin_owner_record_address, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];

    let instruction = ShihonInstruction::Deny;

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Candidate instruction after accepted KickerCoin
#[allow(clippy::too_many_arguments)]
pub fn candidate(
    program_id: &Pubkey,
    // Accounts
    payer: &Pubkey,
    tanistry_account: &Pubkey,
    candidate_token_source: &Pubkey,
    candidate_token_holding: &Pubkey,
    candidate_token_transfer_authority: &Pubkey,
    candidate_token_owner_record: &Pubkey,
    tanistry_token_holding: &Pubkey,
    // Args
    amount: u64,
    governing_token_mint: &Pubkey,
) -> Instruction {
    let candidate_token_owner_record_address = get_candidate_token_owner_record_address(
        program_id,
        tanistry_account,
        candidate_token_source,
        candidate_token_transfer_authority,
    );

    let candidate_token_holding_address = get_tanistry_token_holding_address(
        program_id,
        tanistry_account,
        candidate_token_owner_record,
    );

    let accounts = vec![
        AccountMeta::new_readonly(*tanistry_account, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new(*candidate_token_source, false),
        AccountMeta::new_readonly(*candidate_token_holding, true),
        AccountMeta::new_readonly(*candidate_token_transfer_authority, true),
        AccountMeta::new(kicker_coin_owner_record_address, false),
        AccountMeta::new(tanistry_token_owner_record, true),
        AccountMeta::new_readonly(*tanistry_token_holding, true),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let instruction = ShihonInstruction::Candidate { amount };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Rating other mixed content instruction
pub fn rate_content(
    program_id: &Pubkey,
    // Accounts
    rater_candidate: &Pubkey,
    rater_token_owner: &Pubkey,
    buddy_content_holder: &Pubkey,
    init_content_holder: &Pubkey,
    coordinator: &Pubkey,
    rater_token_mint_for_rating: &Pubkey,
    mix_content_record: &Pubkey,
    // Args
    amount: u64,
) -> Instruction {
    let rate_other_record_address = get_rate_other_record_address(
        program_id,
        rater_candidate,
        rater_token_mint_for_rating,
        init_content_holder,
    );

    let accounts = vec![
        AccountMeta::new(*rater_candidate, true),
        AccountMeta::new_readonly(*rater_token_owner, false),
        AccountMeta::new(*buddy_content_holder false),
        AccountMeta::new(*init_content_holder false),
        AccountMeta::new_readonly(coordinator, false),
        AccountMeta::new(*mix_content_record, false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];

    let instruction = ShihonInstruction::Rate { amount };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Bump self rating instruction
#[allow(clippy::too_many_arguments)]
pub fn bump_self_rate(
    program_id: &Pubkey,
    // Accounts
    candidate_account: &Pubkey,
    candidate_token_source: &Pubkey,
    candidate_token_owner: &Pubkey,
    candidate_token_transfer_authority: &Pubkey,
    payer: &Pubkey,
    candidate_token_owner_record: &Pubkey,
    // Args
    amount: u64,
) -> Instruction {
    let candidate_token_owner_record_address = get_candidate_token_owner_record_address(
        program_id,
        candidate_account,
        candidate_token_source,
        candidate_token_owner,
    );

    let candidate_token_holding_address =
        get_candidate_token_holding_address(program_id, candidate_account, candidate_token_owner);

    let accounts = vec![
        AccountMeta::new_readonly(*candidate_account, false),
        AccountMeta::new(candidate_token_holding_address, false),
        AccountMeta::new(*candidate_token_source, false),
        AccountMeta::new_readonly(*candidate_token_owner, true),
        AccountMeta::new(candidate_token_owner_record_address, false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    let instruction = ShihonInstruction::Bump { amount };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// sell instruction
#[allow(clippy::too_many_arguments)]
pub fn sell_exceeded_rate_token(
    program_id: &Pubkey,
    // Accounts
    seller: &Pubkey,
    sellout_mint: &Pubkey,
    sellout_mint_authority: &Pubkey,
    outside_buyer_record: &Pubkey,
    payer: &Pubkey,
    tanistry_authority: &Pubkey,
) -> Instruction {
    let mint_governance_address = get_sellout_mint_address(program_id, seller, sellout_mint);

    let mut accounts = vec![
        AccountMeta::new_readonly(*seller, false),
        AccountMeta::new(mint_governance_address, false),
        AccountMeta::new(*sellout_mint, false),
        AccountMeta::new_readonly(*sellout_mint_authority, true),
        AccountMeta::new_readonly(*outside_buyer_record, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];

    let instruction = ShihonInstruction::Sell;

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// 0. `[signer]` Outside Buyer account
/// 1. `[writable]` TokenSelloutRecord account. PDA seeds: ['tanistry',bcToken, tanistry_token_mint, tanistry_token_owner]
/// 2. `[writable]` The Seller's token account for selling own token as NFT
/// 3. `[writable]` The Buyer's token account to buy NFT of Candidate
/// 4. `[]` The Buyer's token account for the token they will receive refund if won the game
/// 5. `[Writable]` The Tanistry account holding the Tanistry info  PDA seeds: []
/// 6. `[]` The seller token Mint
/// 7. `[]` System program
/// 8. `[]` Clock sysvar
/// 9. `[]` Sysvar Rent
#[allow(clippy::too_many_arguments)]
/// buy instruction
pub fn buy_exceeded_rate_token(
    program_id: &Pubkey,
    // Accounts
    buyer: &Pubkey,
    outside_buyer_record: &Pubkey,
    buyer_token: &Pubkey,
    buyer_token_owner: &Pubkey,
    payer: &Pubkey,
    // Args
    token: &Pubkey,
    amount: u64,
) -> Instruction {
    let token_governance_address = get_outside_buyer_token_address(program_id, buyer, buyer_token);

    let mut accounts = vec![
        AccountMeta::new_readonly(*realm, false),
        AccountMeta::new(token_governance_address, false),
        AccountMeta::new(*governed_token, false),
        AccountMeta::new_readonly(*governed_token_owner, true),
        AccountMeta::new_readonly(*token_owner_record, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(*governance_authority, true),
    ];

    let instruction = ShihonInstruction::Buy { token, amount };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}

/// Choose the crown on Roydamna instruction
pub fn crowning(
    program_id: &Pubkey,
    // Accounts
    coordinator: &Pubkey,
    kicker_coin_authority: &Pubkey,
    kicker_coin_owner_record: &Pubkey,
    // Args
    crown: &Pubkey,
) -> Instruction {
    let kicker_coin_owner_record_address =
        get_kicker_coin_owner_record_address(program_id, crown, coordinator);

    let accounts = vec![
        AccountMeta::new_readonly(*coordinator, true),
        AccountMeta::new(kicker_coin_onwer_record_address, false),
        AccountMeta::new(*kicker_coin_owner_record, false),
        AccountMeta::new_readonly(*governing_token_mint, false),
        AccountMeta::new_readonly(*kicker_coin_authority, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];

    let instruction = ShihonInstruction::Crowning { crown };

    Instruction {
        program_id: *program_id,
        accounts,
        data: instruction.try_to_vec().unwrap(),
    }
}
