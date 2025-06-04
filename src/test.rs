use {
    borsh::BorshDeserialize,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_instruction,
    },
    solana_program_test::*,
    solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    your_crate_name::{instruction::CreateTokenArgs, state::BuyConfig}, // <-- change `your_crate_name`
};

#[tokio::test]
async fn test_create_instruction() {
    let program_id = Pubkey::new_unique();
    let payer = Keypair::new();
    let mint = Keypair::new();

    let mut program_test = ProgramTest::new(
        "your_crate_name", // Change to your crate name
        program_id,
        processor!(your_crate_name::process_instruction),
    );

    // Add lamports to payer
    program_test.add_account(
        payer.pubkey(),
        solana_sdk::account::Account {
            lamports: 1_000_000_000,
            data: vec![],
            owner: solana_sdk::system_program::id(),
            executable: false,
            rent_epoch: 0,
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Derive PDA
    let (pda, _bump) = Pubkey::find_program_address(&[b"pda-token"], &program_id);

    // Create instruction data
    let args = CreateTokenArgs {
        token_decimals: 6,
        token_supply: 1_000_000_000,
    };
    let mut data = vec![0]; // 0 = Create enum tag
    data.extend(args.try_to_vec().unwrap());

    let create_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(solana_program::sysvar::rent::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(pda, false),
            // Add associated token account, ATA program if needed
        ],
        data,
    };

    let mut transaction = Transaction::new_with_payer(
        &[create_instruction],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &mint], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Fetch PDA data and deserialize BuyConfig
    let pda_account = banks_client.get_account(pda).await.unwrap().unwrap();
    let buy_config = BuyConfig::try_from_slice(&pda_account.data).unwrap();

    assert_eq!(buy_config.is_initialized, true);
    assert_eq!(buy_config.price_set, false);
    assert_eq!(buy_config.price, 0);
}