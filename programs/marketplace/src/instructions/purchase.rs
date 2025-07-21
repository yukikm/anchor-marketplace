


#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    pub maker: SystemAccount<'info>

    #[account(
        seeds = [b"marketplace", marketplace.as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub taker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

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
