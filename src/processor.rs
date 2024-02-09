use solana_program::{
    account_info::{next_account_info, AccountInfo}, borsh1::try_from_slice_unchecked, entrypoint::{self, ProgramResult}, msg, program::invoke_signed, program_error::ProgramError, program_pack::IsInitialized, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}
};

use std::convert::TryInto;

use borsh::BorshSerialize;

use crate::instruction::MovieInstruction;

use crate::state::MovieAccountState;

use crate::error::ReviewError;



pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    msg!("process_instruction() called");

    // Unpack called
    let instruction = MovieInstruction::unpack(instruction_data)?;
    // Match against the data struct returned into `instruction` variable
    match instruction {
        MovieInstruction::AddMovieReview { title, rating, description } => {
            // Make a call to `add_move_review` function
            let add_movie_review_result = add_movie_review(program_id, accounts, title, rating, description);
            msg!("add_movie_review_result: {:?}", add_movie_review_result);
        },
        // add UpdateMovieReview to match against our new data structure
        MovieInstruction::UpdateMovieReview { title, rating, description } => {
            // make call to update function that we'll define next
            update_movie_review(program_id, accounts, title, rating, description);
        }
    }




    Ok(())
}



pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult {

    msg!("add_movie_review() called");

    // Logging instruction data that was passed in
    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);


    if rating > 5 || rating < 1 {
        msg!("Rating cannot be higher than 5");
        return Err(ReviewError::InvalidRating.into())
    }


    let total_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into())
    }




    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;


    // signer check, stops reviews submitted impersonating someone else
    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature)
    }



    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes().as_ref(),], program_id);

    // check if PDA matches client
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into())
    }

    

    // OLD: Calculate account size required
    // let account_len: usize = 1 + 1 + (4 + title.len()) + (4 + description.len());
    // changing this to below, in order to just allocate 1000 bytes to the review account, 
    // so they don't have to worry about recalculation when updating their movie review.
    let account_len: usize = 1000;

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    // Create the account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[initializer.clone(), pda_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(), title.as_bytes().as_ref(), &[bump_seed]]],
    )?;

    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data = try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");


    // checks to see if account already exists, the idea is that the movie review has already been created by that user, and the pda has been created.
    msg!("checking if movie account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }



    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");





    Ok(())
}



pub fn update_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String
) -> ProgramResult {

    msg!("update_movie_review() called");

    msg!("Updating movie review...");

    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    if rating > 5 || rating < 1 {
        msg!("Rating cannot be higher than 5");
        return Err(ReviewError::InvalidRating.into())
    }
    
    


    // Get Account iterator
    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;


    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }


    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    msg!("unpacking state account");
    let mut account_data = try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");


    // Derive PDA and check that it matches client
    let (pda, _bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref(), account_data.title.as_bytes().as_ref(),], program_id);

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    
    msg!("checking if movie account is initialized");
    if !account_data.is_initialized() {
        msg!("Account is not initialized");
        return Err(ReviewError::UninitializedAccount.into());
    }


    let total_len: usize = 1 + 1 + (4 + account_data.title.len()) + (4 + description.len());
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into())
    }


    msg!("Review before update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);


    account_data.rating = rating;
    account_data.description = description;

    msg!("Review after update:");
    msg!("Title: {}", account_data.title);
    msg!("Rating: {}", account_data.rating);
    msg!("Description: {}", account_data.description);

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())






}