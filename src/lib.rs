use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, example_mocks::solana_sdk::sysvar::instructions, msg, pubkey::Pubkey
};
#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType{
    Increase(i64),
    Decrease(i64)
}

#[derive(BorshSerialize, BorshDeserialize)]

struct counter{
    counter: i64
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instructions = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = counter::try_from_slice(&acc.data.borrow())?;

    match instructions{
        InstructionType::Increase(amount)=>{
            counter_data.counter += amount;
        }, 
        InstructionType::Decrease(amount)=>{
            counter_data.counter -= amount;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;

    Ok(())
}