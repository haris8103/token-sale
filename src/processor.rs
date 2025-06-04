use {
    
    // mpl_token_metadata::instruction as mpl_instruction,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint,
        entrypoint::ProgramResult,
        msg,
        program::{invoke,invoke_signed},
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        system_instruction::{ transfer as tr, create_account,},
        sysvar::Sysvar,
        program_error::ProgramError,
    },
    spl_token::{instruction as token_instruction, state::Mint, state::Account as TokenAccount},
    spl_associated_token_account::instruction as associated_token_account_instruction,
    borsh::{BorshDeserialize, BorshSerialize},
    crate::state::BuyConfig,
    crate::instruction::{CreateTokenArgs, FirstBuyArgs, BuyArgs,},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum TokenVesting {
    Create(CreateTokenArgs),
    FirstBuy(FirstBuyArgs),
    Buy(BuyArgs),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TokenVesting::try_from_slice(instruction_data)?;

    match instruction {
        TokenVesting::Create(args) => create(program_id, accounts, args),
        TokenVesting::FirstBuy(args) => first_buy(program_id, accounts, args),
        TokenVesting::Buy(args) => buy(program_id, accounts, args),
    }
}

pub fn create(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateTokenArgs,
) -> ProgramResult {
    // let args = CreateTokenArgs::try_from_slice(instruction_data)?;

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    // let _metadata_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let rent = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    // let _token_metadata_program = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let associated_token_account = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    
    let (pda_account_key, bump) =
    Pubkey::find_program_address(&[b"pda-token"], program_id);
    if pda_account_key != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }
    let signers_seeds: &[&[u8]; 2] = &[b"pda-token",  &[bump]];
    if pda_account.lamports() == 0 {
        msg!("Creating pda account...");
        invoke_signed(
            &create_account(
            payer.key,
            pda_account.key,
            (Rent::get()?).minimum_balance(BuyConfig::LEN),
            BuyConfig::LEN as u64,
            program_id,
            ),
            &[
                pda_account.clone(),
                payer.clone(),
                system_program.clone(),
            ],
            &[signers_seeds]
            
        )?;
    } else {
        msg!("pda account exists.");
    }
    
    // First create the account for the Mint
    //
    msg!("Creating mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &create_account(
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
    //
    msg!("Initializing mint account...");
    msg!("Mint: {}", mint_account.key);
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            pda_account.key,
            Some(pda_account.key),
            args.token_decimals,
        )?,
        &[
            mint_account.clone(),
            pda_account.clone(),
            token_program.clone(),
            rent.clone(),
        ],
    )?;

    // Now create the account for that Mint's metadata
    //
    msg!("Creating metadata account...");
    

    msg!("Token mint created successfully. pda {}, ata {}, mint {}, payer {}", pda_account.key, associated_token_account.key, mint_account.key, payer.key);

    

    if associated_token_account.lamports() == 0 {
        msg!("Creating associated token account...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                pda_account.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                associated_token_account.clone(),
                payer.clone(),
                pda_account.clone(),
                mint_account.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    msg!("Associated Token Address: {}", associated_token_account.key);

    msg!(
        "Minting {} tokens to associated token account...",
        args.token_supply
    );
    invoke_signed(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            pda_account.key,
            &[pda_account.key],
            args.token_supply,
        )?,
        &[
            mint_account.clone(),
            pda_account.clone(),
            associated_token_account.clone(),
            token_program.clone(),
        ],
        &[signers_seeds],
    )?;

    msg!("Tokens minted to wallet successfully.");

    invoke_signed(
        &token_instruction::set_authority(
            token_program.key,
            mint_account.key,
            None,
            token_instruction::AuthorityType::MintTokens,
            pda_account.key,
            &[pda_account.key],
        )?,
        &[
            mint_account.clone(),
            pda_account.clone(),
            token_program.clone(),
        ],
        &[signers_seeds],
    )?;

    msg!("Disabling future minting of this NFT...");
    let buy_config = BuyConfig {
        price: 0u64,
        price_set: false,
        is_initialized: true,
    };
    let mut data = pda_account.data.borrow_mut();
    msg!("data header packed");
    buy_config.pack_into_slice(&mut data);

    Ok(())
}

pub fn first_buy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: FirstBuyArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let associated_token_account = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    
    if destination_token_account.lamports() == 0 {
        msg!("Creating associated token account...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                payer.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                destination_token_account.clone(),
                payer.clone(),
                mint_account.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    let (pda_account_key, bump) =
    Pubkey::find_program_address(&[b"pda-token"], program_id);
    if pda_account_key != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }
    let signers_seeds: &[&[u8]; 2] = &[b"pda-token",  &[bump]];
    let ata_data = TokenAccount::unpack_from_slice(&associated_token_account.try_borrow_data()?)?;
    {
        let pda_data_state = BuyConfig::unpack(&pda_account.try_borrow_data()?)?;
        if pda_data_state.price_set {
            msg!("price set");
            return Err(ProgramError::InvalidInstructionData);
        }
    }
    msg!("Token balance in ATA: {}", ata_data.amount);
    
    let token_amount =  ata_data.amount/100 *(args.bps as u64);
    if token_amount == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let transfer_tokens_from_vesting_account = token_instruction::transfer(
            token_program.key,
            associated_token_account.key,
            destination_token_account.key,
            &pda_account.key,
            &[],
            token_amount,
        )?;

        invoke_signed(
            &transfer_tokens_from_vesting_account,
            &[

                token_program.clone(),
                associated_token_account.clone(),
                destination_token_account.clone(),
                pda_account.clone(),
            ],
            &[signers_seeds]
        )?;

        invoke(
            &tr(payer.key, pda_account.key, args.amount),
            &[
                payer.clone(),
                pda_account.clone(),
                system_program.clone(),
            ],
        )?;
    let mut pda_data = pda_account.data.borrow_mut();
    let mut pda_data_state = BuyConfig::unpack(&pda_data)?;
    pda_data_state.price = args.amount/token_amount;
    pda_data_state.price_set = true;
    msg!("{}, {}, {}", pda_data_state.price, args.amount, token_amount);
    pda_data_state.pack_into_slice(&mut pda_data);
    Ok(())
}

pub fn buy(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: BuyArgs,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let associated_token_account = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;
    let destination_token_account = next_account_info(accounts_iter)?;
    let (pda_account_key, bump) =
    Pubkey::find_program_address(&[b"pda-token"], program_id);
    if pda_account_key != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }
    let mut token_amount = 0;
    let signers_seeds: &[&[u8]; 2] = &[b"pda-token",  &[bump]];
    {
        let pda_data_state = BuyConfig::unpack(&pda_account.try_borrow_data()?)?;
        
        if pda_data_state.price == 0 {
            msg!("Price is zero");
            return Err(ProgramError::InvalidInstructionData);
        }
        token_amount =  args.amount / pda_data_state.price;
    }

    if destination_token_account.lamports() == 0 {
        msg!("Creating associated token account...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                payer.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                destination_token_account.clone(),
                payer.clone(),
                mint_account.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    
    if token_amount == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let transfer_tokens_from_vesting_account = token_instruction::transfer(
            token_program.key,
            associated_token_account.key,
            destination_token_account.key,
            &pda_account.key,
            &[],
            token_amount,
            
        )?;

        invoke_signed(
            &transfer_tokens_from_vesting_account,
            &[
                token_program.clone(),
                associated_token_account.clone(),
                destination_token_account.clone(),
                pda_account.clone(),
            ],
            &[signers_seeds]
        )?;

        invoke(
            &tr(payer.key, pda_account.key, args.amount),
            &[
                payer.clone(),
                pda_account.clone(),
                system_program.clone(),
            ],
        )?;
    
    Ok(())
}