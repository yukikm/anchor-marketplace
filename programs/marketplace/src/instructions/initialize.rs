use anchor_lang::{accounts::signer, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::FreezeDelegatedAccountCpi, MetadataAccount}, token::{approve, transfer, Approve, Mint, Token, TokenAccount, Transfer}
};
use crate::error::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[
        account(
            init,
            payer = admin,
            seeds = [b"marketplace", name.as_ref()],
            bump,
            space = 8 + Marketplace::INIT_SPACE,
        )
    ]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[
        account(
            init,
            payer = admin,
            seeds = [b"rewards", marketplace.key().as_ref()],
            bump,
            mint::decimals = 6,
            mint::authority = marketplace,
        )
    ]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_interface: Program<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(!name.is_empty() && name.len() < 4 + 33, MarketplaceError::NameTooLong);
    }
}
