use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::instruction::{
    mint_new_edition_from_master_edition_via_token, update_primary_sale_happened_via_token,
};
use solana_program::program::invoke;

pub fn prinNewNft(ctx: Context<PrinNewNFTContext>) -> Result<()> {
    invoke(
        &mint_new_edition_from_master_edition_via_token(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.new_metadata_account.key(),
            ctx.accounts.new_edition_account.key(),
            ctx.accounts.master_edition_account.key(),
            ctx.accounts.new_mint.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.token_account.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.mint.key(),
            1,
        ),
        &[
            ctx.accounts.new_metadata_account.to_account_info(),
            ctx.accounts.new_edition_account.to_account_info(),
            ctx.accounts.master_edition_account.to_account_info(),
            ctx.accounts.new_mint.to_account_info(),
            ctx.accounts.edition_mark_pda.to_account_info(),
            ctx.accounts.nft_creator.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    invoke(
        &update_primary_sale_happened_via_token(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.token_account.key(),
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.nft_creator.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
        ],
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct PrinNewNFTContext<'info> {
    #[account(mut)]
    pub nft_creator: Signer<'info>,

    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        mint::authority = nft_creator,
    )]
    pub new_mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = nft_creator,
    )]
    pub token_account: Account<'info, TokenAccount>,
    #[account(
        associated_token::mint = new_mint,
        associated_token::authority = nft_creator,
    )]
    pub new_token_account: Account<'info, TokenAccount>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub new_metadata_account: AccountInfo<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub new_edition_account: AccountInfo<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub edition_mark_pda: AccountInfo<'info>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
