use crate::error::*;
use crate::state::{InitializeBumps, Marketplace};
use anchor_lang::{accounts::signer, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{mpl_token_metadata::instructions::FreezeDelegatedAccountCpi, MetadataAccount},
    token::{approve, transfer, Approve, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        seeds = [marketplace.as_ref(), maker_mint.key().as_ref()]
        bump,
        space = 8 + Listing::INIT_SPACE,
    )]
    pub listing: Account<'info, Listing>,

    #[
        account(
            seeds = [b"treasury", marketplace.key().as_ref()],
        )
    ]
    pub metadata_program: Program<'info, Metadata>,
}
