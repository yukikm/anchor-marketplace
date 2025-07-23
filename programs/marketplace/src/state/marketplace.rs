use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,     // the admin of the marketplace
    pub fee: u16,          // the fee percentage for the marketplace
    pub bump: u8,          // the bump seed for the marketplace account
    pub treasury_bump: u8, // the bump seed for the treasury account
    pub rewards_bump: u8,  // the bump seed for the rewards account
    #[max_len(32)]
    pub name: String, // the name of the marketplace
}
