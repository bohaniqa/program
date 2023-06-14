use {
    solana_program::{
        decode_error::DecodeError, 
        program_error::ProgramError,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BOQError {}

impl From<BOQError> for ProgramError {
    fn from(e: BOQError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for BOQError {
    fn type_of() -> &'static str {
        "BOQ Error"
    }
}