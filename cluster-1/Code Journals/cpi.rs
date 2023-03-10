//importing required methods and structs
use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    program::invoke,
};

//entrypoint of the program
entrypoint!(pull_lever);


fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                         
        &set_power_status_instruction,                      
        vec![AccountMeta::new(power.key.clone(), false)],   
    );

    invoke(&ix, &[power.clone()])
}