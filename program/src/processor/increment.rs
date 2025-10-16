use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub fn process_increment(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [counter_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut data = counter_account.try_borrow_mut_data()?;
    data[0] = data[0].saturating_add(1);

    Ok(())
}
