use crate::state::{Listing, Marketplace};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    maker_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        // listing account is automatically closed and funds are transferred to the maker
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    listing: Box<Account<'info, Listing>>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        // listing account
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }
}
