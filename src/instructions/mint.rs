// #![allow(deprecated)]

use {
    mpl_token_metadata::instructions::CreateMasterEditionV3Builder,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::invoke,
    },
    spl_associated_token_account::instruction as ata_instruction,
    // spl_associated_token_account::create_associated_token_account,
    spl_token::instruction as token_instruction,
};

pub fn mint_to(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let associated_token_account = next_account_info(accounts_iter)?;
    let _associated_token_program = next_account_info(accounts_iter)?;
    let edition_account = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let mint_authority = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_metadata_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let wallet = next_account_info(accounts_iter)?;

    if associated_token_account.lamports() != 0 {
        msg!("Associated token account exists.");
    } else {
        msg!("Creating associated token account...");

        // For debugging purposes only!
        // ```rust
        // msg!(
        //     "\nata: {}\natp: {}\nea: {},\nmda: {}\nmint: {}\nmauth: {}\n\
        //     payer: {}\nrent: {}\nsysprog: {}\ntmprog: {}\ntprog: {}\n\
        //     wallet: {}",
        //     associated_token_account.key,
        //     associated_token_program.key,
        //     edition_account.key,
        //     metadata_account.key,
        //     mint_account.key,
        //     mint_authority.key,
        //     payer.key,
        //     rent.key,
        //     system_program.key,
        //     token_metadata_program.key,
        //     token_program.key,
        //     wallet.key,
        // );
        // ```

        // TODO: Resolve the issue with `create_associated_token_account()`
        // invocation
        invoke(
            &ata_instruction::create_associated_token_account(
                payer.key,
                wallet.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                payer.clone(),
                associated_token_account.clone(),
                wallet.clone(),
                mint_account.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;
    }
    msg!("Associated Token Address: {}", associated_token_account.key);

    // Mint the NFT to the user's wallet
    msg!("Minting NFT to associated token account...");
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            mint_authority.key,
            &[mint_authority.key],
            1,
        )?,
        &[
            mint_account.clone(),
            mint_authority.clone(),
            associated_token_account.clone(),
            token_program.clone(),
        ],
    )?;

    // We can make this a Limited Edition NFT through Metaplex,
    // which will disable minting by setting the Mint & Freeze Authorities to
    // the Edition Account
    msg!("Creating edition account...");
    msg!("Edition account address: {}", edition_account.key);
    invoke(
        &CreateMasterEditionV3Builder::new()
            .edition(*edition_account.key)
            .mint(*mint_account.key)
            .update_authority(*mint_authority.key)
            .mint_authority(*mint_authority.key)
            .payer(*payer.key)
            .metadata(*metadata_account.key)
            .token_program(*token_program.key)
            .system_program(*system_program.key)
            .rent(Some(*rent.key))
            .max_supply(1)
            .instruction(),
        &[
            edition_account.clone(),
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            token_metadata_program.clone(),
            rent.clone(),
        ],
    )?;

    // If we don't use Metaplex Editions, we must disable minting manually as
    // follows:
    // ```rust
    // msg!("Disabling future minting of this NFT...");
    // invoke(
    //     &token_instruction::set_authority(
    //         &token_program.key,
    //         &mint_account.key,
    //         None,
    //         token_instruction::AuthorityType::MintTokens,
    //         &mint_authority.key,
    //         &[&mint_authority.key],
    //     )?,
    //     &[
    //         mint_account.clone(),
    //         mint_authority.clone(),
    //         token_program.clone(),
    //     ],
    // )?;
    // invoke(
    //     &token_instruction::set_authority(
    //         &token_program.key,
    //         &mint_account.key,
    //         None,
    //         token_instruction::AuthorityType::FreezeAccount,
    //         &mint_authority.key,
    //         &[&mint_authority.key],
    //     )?,
    //     &[
    //         mint_account.clone(),
    //         mint_authority.clone(),
    //         token_program.clone(),
    //     ],
    // )?;
    // ```

    msg!("NFT minted successfully!");

    Ok(())
}
