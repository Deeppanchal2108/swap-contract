use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer{
    pub id:u64,
    pub maker:Pubkey,
    pub token_mint_a:Pubkey,
    pub token_mint_b:Pubkey,
    pub number_of_token_b_wanted:u64,
    pub bump:u8
}