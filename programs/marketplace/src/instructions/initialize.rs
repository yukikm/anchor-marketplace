use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::error::MarketplaceError;
use crate::state::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        space = Marketplace::INIT_SPACE,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    // if large size accounts are used, use Box to avoid stack overflow
    // https://solana.stackexchange.com/questions/4926/when-and-why-to-use-boxed-accounts

    // reward mint account
    #[account(
        init,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        payer = admin,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    rewards_mint: Box<InterfaceAccount<'info, Mint>>,
    // manage treasury assets in the marketplace
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    treasury: SystemAccount<'info>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(
            name.len() > 0 && name.len() < 33,
            MarketplaceError::NameTooLong
        );
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            name,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
        });

        Ok(())
    }
}
