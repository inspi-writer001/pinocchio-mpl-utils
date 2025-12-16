pub mod initialize;
pub use initialize::*;

#[repr(u8)]
pub enum PinocchioInstruction {
    Initialize,
}

impl TryFrom<&u8> for PinocchioInstruction {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PinocchioInstruction::Initialize),

            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}
