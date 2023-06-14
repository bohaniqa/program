// The deployed program id.
solana_program::declare_id!("J1FMqW26pFkvgqezcS58DEuKgVPsMcPr7P2SugrBBbqa");

#[cfg(not(feature = "no-entrypoint"))]
pub mod check;
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;