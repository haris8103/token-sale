

// Anchor-compatible version of the original native Solana Rust code

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer, SetAuthority};
use anchor_spl::associated_token::AssociatedToken;
use spl_token::instruction::AuthorityType::MintTokens;

// Define your account state structure here (formerly BuyConfig)
#[account]
pub struct BuyConfig {
    pub price: u64,
    pub price_set: bool,
    pub is_initialized: bool,
}

impl BuyConfig {
    pub const LEN: usize = 8 + 8 + 1 + 1; // Update size for price (u64), bools, and discriminator
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TokenVesting {
    Create(CreateTokenArgs),
    FirstBuy(FirstBuyArgs),
    Buy(BuyArgs),
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(init, payer = payer, space = 8 + BuyConfig::LEN, seeds = [b"pda-token"], bump)]
    pub pda_account: Account<'info, BuyConfig>,

    #[account(init, payer = payer, space = 82)]
    pub mint_account: Account<'info, Mint>,

    #[account(init_if_needed, payer = payer, associated_token::mint = mint_account, associated_token::authority = pda_account)]
    pub associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct FirstBuy<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut, seeds = [b"pda-token"], bump)]
    pub pda_account: Account<'info, BuyConfig>,

    #[account(mut, associated_token::mint = mint_account, associated_token::authority = pda_account)]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    #[account(mut, seeds = [b"pda-token"], bump)]
    pub pda_account: Account<'info, BuyConfig>,

    #[account(mut, associated_token::mint = mint_account, associated_token::authority = pda_account)]
    pub associated_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,
}

pub fn create(ctx: Context<CreateToken>, args: CreateTokenArgs) -> Result<()> {
    let seeds: &[&[u8]] = &[b"pda-token", &[ctx.bumps.pda_account]];
    msg!("Hello");
    let init_mint_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::InitializeMint {
            mint: ctx.accounts.mint_account.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );
    token::initialize_mint(
        init_mint_ctx,
        args.token_decimals,
        &ctx.accounts.pda_account.key(),
        Some(&ctx.accounts.pda_account.key())
    )?;
    let binding = [&seeds[..]];
    let mint_to_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.associated_token_account.to_account_info(),
            authority: ctx.accounts.pda_account.to_account_info(),
        },
        &binding,
    );
    token::mint_to(mint_to_ctx, args.token_supply)?;
   
    let set_auth_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            current_authority: ctx.accounts.pda_account.to_account_info(),
            account_or_mint: ctx.accounts.mint_account.to_account_info(),
        },
        &binding,
    );
    token::set_authority(
        set_auth_ctx,
        MintTokens,
        None,
    )?;

    let config = &mut ctx.accounts.pda_account;
    config.price = 0;
    config.price_set = false;
    config.is_initialized = true;
    Ok(())
}

pub fn first_buy(ctx: Context<FirstBuy>, args: FirstBuyArgs) -> Result<()> {
    let seeds: &[&[u8]] = &[b"pda-token", &[ctx.bumps.pda_account]];
    let vesting_account = &ctx.accounts.associated_token_account;
    let dest_account = &ctx.accounts.destination_token_account;

    let token_amount = vesting_account.amount / 100 * args.bps as u64;
    require!(token_amount > 0, ErrorCode::ZeroTokenAmount);
    let binding = [&seeds[..]];
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: vesting_account.to_account_info(),
            to: dest_account.to_account_info(),
            authority: ctx.accounts.pda_account.to_account_info(),
        },
        &binding,
    );
    token::transfer(transfer_ctx, token_amount)?;

    **ctx.accounts.pda_account.to_account_info().try_borrow_mut_lamports()? += args.amount;
    **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= args.amount;

    let config = &mut ctx.accounts.pda_account;
    require!(!config.price_set, ErrorCode::PriceAlreadySet);
    config.price = args.amount / token_amount;
    config.price_set = true;
    Ok(())
}

pub fn buy(ctx: Context<Buy>, args: BuyArgs) -> Result<()> {
    let seeds: &[&[u8]] = &[b"pda-token", &[ctx.bumps.pda_account]];
    let config = &ctx.accounts.pda_account;
    require!(config.price > 0, ErrorCode::ZeroPrice);

    let token_amount = args.amount / config.price;
    require!(token_amount > 0, ErrorCode::ZeroTokenAmount);
    let binding = [&seeds[..]];
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.associated_token_account.to_account_info(),
            to: ctx.accounts.destination_token_account.to_account_info(),
            authority: ctx.accounts.pda_account.to_account_info(),
        },
        &binding,
    );
    token::transfer(transfer_ctx, token_amount)?;

    **ctx.accounts.pda_account.to_account_info().try_borrow_mut_lamports()? += args.amount;
    **ctx.accounts.payer.to_account_info().try_borrow_mut_lamports()? -= args.amount;
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateTokenArgs {
    pub token_supply: u64,
    pub token_decimals: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct FirstBuyArgs {
    pub bps: u16,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BuyArgs {
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token amount must be greater than 0")]
    ZeroTokenAmount,
    #[msg("Price is zero")]
    ZeroPrice,
    #[msg("Price already set")]
    PriceAlreadySet,
}
