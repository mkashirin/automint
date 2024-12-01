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
enum SplNftMinterIntstruction {
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

#[cfg(test)]
mod test {
    use {
        solana_program::{instruction::Instruction, msg},
        solana_program_test::*,
        solana_sdk::{
            instruction::AccountMeta,
            signature::{Keypair, Signer},
            system_program,
            sysvar::rent,
            transaction::Transaction,
        },
        spl_associated_token_account::get_associated_token_address_with_program_id,
    };

    use super::*;

    fn initialize_program_test() -> (Pubkey, solana_program_test::ProgramTest) {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "mintingnft",
            program_id,
            processor!(process_instruction),
        );
        program_test.add_program(
            "metaplex_token_metadata_program",
            mpl_token_metadata::ID,
            None,
        );
        (program_id, program_test)
    }

    #[ignore = "Passes. Need to only run `test_mint_created_nft_token()`"]
    #[tokio::test]
    async fn test_create_nft_token() {
        msg!("Running `test_create_nft_token`...");

        // Setting up the testing environment
        let (program_id, program_test) = initialize_program_test();
        let ctx = program_test.start_with_context().await;
        let mint_authority_id = ctx.payer.pubkey();
        let payer_id = ctx.payer.pubkey();

        let mint_account = Keypair::new();
        let mint_account_id = mint_account.pubkey();
        let (metadata_account_id, _) = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                mpl_token_metadata::ID.as_ref(),
                &mint_account_id.to_bytes(),
            ],
            &mpl_token_metadata::ID,
        );

        // Initializing instruction
        let ix = Instruction::new_with_borsh(
            program_id,
            &SplNftMinterIntstruction::Create(CreateNftTokenArgs {
                name: "Ballet Dancers BTC".to_string(),
                symbol: "BDB".to_string(),
                uri: "https://storage.yandexcloud.net/lab-bucket/item.json"
                    .to_string(),
            }),
            vec![
                AccountMeta::new(metadata_account_id, false),
                AccountMeta::new(mint_account_id, true),
                AccountMeta::new(mint_authority_id, true),
                AccountMeta::new(payer_id, true),
                AccountMeta::new_readonly(rent::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(mpl_token_metadata::ID, false),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
        );

        // Initializing transaction with the instruction above
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_id),
            &[&ctx.payer, &mint_account],
            ctx.last_blockhash,
        );

        // Executing the transaction
        let tx_result = ctx.banks_client.process_transaction(tx).await;

        assert!(tx_result.is_ok());
    }

    #[tokio::test]
    async fn test_mint_created_nft_token() {
        msg!("Running `test_mint_created_nft_token`...");

        // Setting up the testing environment
        let (program_id, program_test) = initialize_program_test();
        msg!("Program ID: {}", program_id);
        let ctx = program_test.start_with_context().await;
        let mint_authority_id = ctx.payer.pubkey();
        let payer_id = ctx.payer.pubkey();

        let mint_account = Keypair::new();
        let mint_account_id = mint_account.pubkey();
        let wallet = Keypair::new();
        let wallet_id = wallet.pubkey();
        let (edition_account_id, _) = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                mpl_token_metadata::ID.as_ref(),
                &mint_account_id.to_bytes(),
                "edition".as_bytes(),
            ],
            &mpl_token_metadata::ID,
        );
        let (metadata_account_id, _) = Pubkey::find_program_address(
            &[
                "metadata".as_bytes(),
                mpl_token_metadata::ID.as_ref(),
                &mint_account_id.to_bytes(),
            ],
            &mpl_token_metadata::ID,
        );
        let associated_token_account_id =
            get_associated_token_address_with_program_id(
                &payer_id,
                &mint_account_id,
                &spl_token::ID,
            );

        // Create NFT
        msg!("Creating NFT...");

        // Initializing instruction to create NFT
        let ix = Instruction::new_with_borsh(
            program_id,
            &SplNftMinterIntstruction::Create(CreateNftTokenArgs {
                name: "Ballet Dancers BTC".to_string(),
                symbol: "BDB".to_string(),
                uri: "https://storage.yandexcloud.net/lab-bucket/item.json"
                    .to_string(),
            }),
            vec![
                AccountMeta::new(metadata_account_id, false),
                AccountMeta::new(mint_account_id, true),
                AccountMeta::new(mint_authority_id, true),
                AccountMeta::new(payer_id, true),
                AccountMeta::new_readonly(rent::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(mpl_token_metadata::ID, false),
                AccountMeta::new_readonly(spl_token::ID, false),
            ],
        );

        // Initializing transaction with the instruction above
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_id),
            &[&ctx.payer, &mint_account],
            ctx.last_blockhash,
        );

        // Executing the transaction above
        let tx_result = ctx.banks_client.process_transaction(tx).await;

        assert!(tx_result.is_ok());
        msg!("NFT created successfully!");

        // Mint the created NFT
        msg!("Minting NFT...");

        // Initializing instruction to mint the created NFT
        let ix = Instruction::new_with_borsh(
            program_id,
            &SplNftMinterIntstruction::Mint,
            vec![
                AccountMeta::new(associated_token_account_id, false),
                AccountMeta::new_readonly(
                    spl_associated_token_account::ID,
                    false,
                ),
                AccountMeta::new(edition_account_id, false),
                AccountMeta::new(metadata_account_id, false),
                AccountMeta::new(mint_account_id, true),
                AccountMeta::new(mint_authority_id, false),
                AccountMeta::new(payer_id, true),
                AccountMeta::new_readonly(rent::ID, false),
                AccountMeta::new_readonly(system_program::ID, false),
                AccountMeta::new_readonly(spl_token::ID, false),
                AccountMeta::new_readonly(mpl_token_metadata::ID, false),
                AccountMeta::new_readonly(wallet_id, false),
            ],
        );

        // Initializing transaction with the instruction above
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&payer_id),
            &[&ctx.payer, &mint_account],
            ctx.last_blockhash,
        );

        // Executing the transaction above
        let tx_result = ctx.banks_client.process_transaction(tx).await;

        assert!(tx_result.is_ok());
        msg!("NFT minted successfully!");
    }
}
