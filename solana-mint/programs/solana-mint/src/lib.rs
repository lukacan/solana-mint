use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;
declare_id!("BH4PdDrrKvnrisdxmdTftf8rxPzmwN74jbXmKanSDhyo");

#[program]
pub mod solana_mint {

    use super::*;

    pub fn create_nft(ctx: Context<CreateNftcontext>) -> Result<()> {
        instructions::create_nft(ctx)
    }
    pub fn print_new_nft(ctx: Context<PrinNewNFTContext>) -> Result<()> {
        instructions::prinNewNft(ctx)
    }
}
