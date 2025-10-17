use shank::ShankInstruction;

/// Instructions for myproject. This is currently not used in the
/// program business logic, but we include it for IDL generation.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, ShankInstruction)]
pub enum MyProjectInstruction {
    Initialize,
    Increment,
}

impl TryFrom<&u8> for MyProjectInstruction {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MyProjectInstruction::Initialize),
            1 => Ok(MyProjectInstruction::Increment),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}
