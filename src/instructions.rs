use borsh::BorshDeserialize;
use borsh::BorshSerialize;

use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => {
                let value = u32::from_le_bytes(
                    rest.get(0..4)
                        .ok_or(ProgramError::InvalidInstructionData)?
                        .try_into()
                        .unwrap(),
                );
                Ok(CounterInstructions::Increment(value))
            }
            1 => {
                let value = u32::from_le_bytes(
                    rest.get(0..4)
                        .ok_or(ProgramError::InvalidInstructionData)?
                        .try_into()
                        .unwrap(),
                );
                Ok(CounterInstructions::Decrement(value))
            }
            2 => {
                let args = UpdateArgs::try_from_slice(rest)?;
                Ok(CounterInstructions::Update(args))
            }
            3 => Ok(CounterInstructions::Reset),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}