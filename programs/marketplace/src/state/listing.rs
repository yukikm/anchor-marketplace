use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub maker: Pubkey, // the creator of the listing
    pub mint: Pubkey,  // the mint address of the token being listed
    pub price: u64,    // the price of the listing in lamports
    pub bump: u8,      // the bump seed for the listing account
}
