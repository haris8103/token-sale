use anchor_lang::prelude::*;

pub mod processor;
pub use processor::*;

declare_id!("kJzY1czYx9MZZ1Djjr3Npk34ahmm6BBCNJqZy4gfn7r");

#[program]
pub mod token_sale_program {
    use super::*;

    pub fn create(ctx: Context<CreateToken>, args: CreateTokenArgs) -> Result<()> {
        processor::create(ctx, args)
    }

    pub fn first_buy(ctx: Context<FirstBuy>, args: FirstBuyArgs) -> Result<()> {
        processor::first_buy(ctx, args)
    }

    pub fn buy(ctx: Context<Buy>, args: BuyArgs) -> Result<()> {
        processor::buy(ctx, args)
    }
}
