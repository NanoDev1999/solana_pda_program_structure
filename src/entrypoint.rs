use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::AccountInfo,
    pubkey::Pubkey,
    msg
};


use crate::processor;





entrypoint!(process_instruction);



pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult{
    msg!("process_instruction() called");

    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    processor::process_instruction(program_id, accounts, instruction_data)?;




    Ok(())
}
