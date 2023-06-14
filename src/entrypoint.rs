use {
    crate::{
        processor::Processor,
    },
    solana_program::{
        account_info::AccountInfo,
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        pubkey::Pubkey,
    },
};

// Call the entry point macro to make `process_instruction` the entry point to our program.
entrypoint!(process_instruction);

// The program's entry point. A solana program has one entry point and it's a convention to name it 
// `process_instruction`.
fn process_instruction(
    // The program's id on the solana network.
    program_id: &Pubkey,
    // The accounts used by the instruction being invoked.
    accounts: &[AccountInfo],
    // The data passed to the instruction being invoked.
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        msg!("[Processor] Error: {}", error); // Print the error for debugging.
        Err(error)
    } else {
        Ok(())
    }
}