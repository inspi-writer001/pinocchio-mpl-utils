// #[allow(unexpected_cfgs, unused)]
use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::instruction::{process_intialize, PinocchioInstruction};

mod instruction;
mod state;
mod test;

entrypoint!(process_instruction);
pinocchio_pubkey::declare_id!("9XZcaw8CScYtsgfs7sw76qCqddycy7y41BiATa62KygN");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match PinocchioInstruction::try_from(discriminator)? {
        PinocchioInstruction::Initialize => process_intialize(accounts, data)?,
        _ => return Err(ProgramError::InvalidInstructionData),
    }
    Ok(())
}
