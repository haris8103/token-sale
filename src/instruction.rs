
use {
    solana_program::{
        
        msg,
        program_error::ProgramError,
        pubkey::Pubkey
    },
    borsh::{BorshDeserialize, BorshSerialize},
};

use std::convert::TryInto;
use std::mem::size_of;


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateTokenArgs {
    pub token_decimals:  u8,
    pub token_supply: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct FirstBuyArgs {
    pub bps:  u8,
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BuyArgs {
    pub amount:  u64,
}