use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};


use crate::{Offer, ANCHOR_DISCRIMINATOR};

use super::transfer_tokens;

#[derive(Accounts)]
pub struct MakeOffer <'info>{

    #[account(mut)]
    pub maker:Signer<'info>,

    pub token_mint_a:InterfaceAccount<'info,Mint>,
    pub token_mint_b:InterfaceAccount<'info,Mint>,




}