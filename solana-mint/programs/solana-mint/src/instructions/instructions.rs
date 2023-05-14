use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::instruction::{
    create_master_edition_v3, create_metadata_accounts_v3, sign_metadata,
};
use solana_program::program::invoke;

pub fn create_nft(ctx: Context<CreateNftcontext>) -> Result<()> {
    let creator = vec![mpl_token_metadata::state::Creator {
        address: ctx.accounts.nft_creator.key(),
        verified: false,
        share: 100,
    }];

    // create metadata account
    invoke(
        &create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.nft_creator.key(),
            std::string::ToString::to_string("Andrej"),
            std::string::ToString::to_string("Symbol"),
            std::string::ToString::to_string("Uri"),
            Some(creator),
            1,
            true,
            true,
            None,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.nft_creator.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;

    // sign with creator
    invoke(
        &sign_metadata(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.nft_creator.key(),
        ),
        &[
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.nft_creator.to_account_info(),
        ],
    )?;

    // create master edition
    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition_account.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.nft_creator.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.nft_creator.key(),
            Some(10),
        ),
        &[
            ctx.accounts.master_edition_account.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.nft_creator.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct CreateNftcontext<'info> {
    #[account(mut)]
    pub nft_creator: Signer<'info>,
    #[account(
        mut,
        mint::authority = nft_creator,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = nft_creator,
    )]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition_account: AccountInfo<'info>,

    /// CHECK: Metaplex will check this
    pub token_metadata_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
