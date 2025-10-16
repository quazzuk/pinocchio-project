#[cfg(test)]
mod tests {
    use mollusk_svm::{Mollusk, result::ProgramResult};
    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    };

    #[test]
    fn test_initialize() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "arboor");

        let counter = Pubkey::new_unique();
        let mut counter_account = AccountSharedData::new(100, 1, &program_id);
        counter_account.data_as_mut_slice()[0] = 255;

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[0],
            vec![AccountMeta::new(counter, false)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &[(counter, counter_account)],
        );

        assert_eq!(result.program_result, ProgramResult::Success);
        assert_eq!(result.resulting_accounts[0].1.data()[0], 0);
    }

    #[test]
    fn test_increment() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "arboor");

        let counter = Pubkey::new_unique();
        let mut counter_account = AccountSharedData::new(100, 1, &program_id);
        counter_account.data_as_mut_slice()[0] = 5;

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[1],
            vec![AccountMeta::new(counter, false)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &[(counter, counter_account)],
        );

        assert_eq!(result.program_result, ProgramResult::Success);
        assert_eq!(result.resulting_accounts[0].1.data()[0], 6);
    }

    #[test]
    fn test_invalid_instruction() {
        let program_id = Pubkey::new_unique();
        let mollusk = Mollusk::new(&program_id, "arboor");

        let counter = Pubkey::new_unique();
        let counter_account = AccountSharedData::new(100, 1, &program_id);

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[2],
            vec![AccountMeta::new(counter, false)],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &[(counter, counter_account)],
        );

        assert!(matches!(result.program_result, ProgramResult::Failure(_)));
    }
}