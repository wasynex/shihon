pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod tools;

pub use solana_program;

/// Seed prefix for shihon PDAs
pub const PROGRAM_AUTHORITY_SEED: &[u8] = b"shihon";
