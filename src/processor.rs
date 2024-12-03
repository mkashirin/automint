use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
    },
};

use crate::instructions::{
    create::{create_token, CreateNftTokenArgs},
    mint::mint_to,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SplNftMinterIntstruction {
    Create(CreateNftTokenArgs),
    Mint,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction =
        SplNftMinterIntstruction::try_from_slice(instruction_data)?;

    match instruction {
        SplNftMinterIntstruction::Create(args) => create_token(accounts, args),
        SplNftMinterIntstruction::Mint => mint_to(accounts),
    }
}
