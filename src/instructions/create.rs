use {
    borsh::{BorshDeserialize, BorshSerialize},
    mpl_token_metadata::{
        instructions::CreateMetadataAccountV3Builder, types::DataV2,
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
        program_pack::Pack,
        rent::Rent,
        system_instruction,
        sysvar::Sysvar,
    },
    spl_token::{instruction as token_instruction, state::Mint},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateNftTokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub fn create_token(
    accounts: &[AccountInfo],
    args: CreateNftTokenArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let metadata_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // First create the account for the Mint
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            (Rent::get()?).minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            token_program.key,
        ),
        &[
            mint_account.clone(),
            payer.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
    )?;

    // Now initialize that account as a Mint (standard Mint)
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            Some(mint_authority.key),
            0,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    // Now create the account for that Mint's metadata
    msg!("Creating metadata account...");
    msg!("Metadata account address: {}", metadata_account.key);
    invoke(
        &CreateMetadataAccountV3Builder::new()
            .metadata(*metadata_account.key)
            .mint(*mint_account.key)
            .mint_authority(*mint_authority.key)
            .payer(*payer.key)
            .update_authority(*mint_authority.key, true)
            .system_program(*system_program.key)
            .rent(Some(*rent.key))
            .data(DataV2 {
                name: args.name,
                symbol: args.symbol,
                uri: args.uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            })
            .is_mutable(true)
            .instruction(),
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
            system_program.clone(),
            rent.clone(),
        ],
    )?;

    msg!("Token mint created successfully.");
    Ok(())
}
