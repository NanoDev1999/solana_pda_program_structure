use solana_program::program_pack::{Sealed, IsInitialized};

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovieAccountState {
    pub is_initialized: bool,
    pub rating: u8,
    pub title: String,
    pub description: String  
}


impl Sealed for MovieAccountState {}    // this specifies that MoveAccountState has a known size. Solana's version of Sized in Rust.

impl IsInitialized for MovieAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

