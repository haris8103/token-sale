use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use std::convert::TryInto;
#[derive(Debug, PartialEq)]
pub struct BuyConfig {
    pub price: u64,
    pub price_set: bool,
    pub is_initialized: bool,
}

impl Sealed for BuyConfig {}

impl Pack for BuyConfig {
    const LEN: usize = 10;

    fn pack_into_slice(&self, target: &mut [u8]) {
        let price = self.price.to_le_bytes();
        target[0..8].copy_from_slice(&price);
        target[8] = self.price_set as u8;
        target[9] = self.is_initialized as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        if src.len() < 10 {
            return Err(ProgramError::InvalidAccountData)
        }
        let price = u64::from_le_bytes(src[0..8].try_into().unwrap());
        let price_set = src[8] == 1;
        let is_initialized = src[9] == 1;
        Ok(Self {
            price,
            price_set,
            is_initialized,
        })
    }
}


impl IsInitialized for BuyConfig {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
