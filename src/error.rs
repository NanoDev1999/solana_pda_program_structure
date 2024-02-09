// Errors Handled:
// The update instruction has been invoked on an account that hasn't been initialized yet
// The provided PDA doesn't match the expected or derived PDA
// The input data is larger than the program allows
// The rating provided does not fall in the 1-5 range


use solana_program::program_error::ProgramError;
use thiserror::Error;



#[derive(Debug, Error)]
pub enum ReviewError{

    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("Input data exceeds max length")]
    InvalidDataLength,

    #[error("Rating greater than 5 or less than 1")]
    InvalidRating,
}


// implementation that lets us convert our error into a ProgramError type as needed.
impl From<ReviewError> for ProgramError {   
    fn from(e: ReviewError) -> Self {   
        ProgramError::Custom(e as u32)
    }
}






