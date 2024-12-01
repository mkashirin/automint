use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

pub mod instructions;
pub mod processor;

solana_program::declare_id!("48rXmSB6NfPUBuQodNpDojvTX6XzYAv7btiqZBexMWQe");

solana_program::entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}
