pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod modules;
pub mod processor;
pub mod state;

pub use solana_program;

/// Seed prefix for bcToken  PDAs
pub const PROGRAM_AUTHORITY_SEED: &[u8] = b"bctoken";
