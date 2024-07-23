use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let account = next_account_info(accounts_iter)?;

    let mut data = account.try_borrow_mut_data()?;
    let counter = &mut data[0];

    match instruction_data[0] {
        0 => {
            *counter = counter.wrapping_add(1);
            msg!("Incremented value: {}", *counter);
        }
        1 => {
            msg!("Current value: {}", *counter);
        }
        _ => {
            msg!("Invalid instruction");
        }
    }

    Ok(())
}
