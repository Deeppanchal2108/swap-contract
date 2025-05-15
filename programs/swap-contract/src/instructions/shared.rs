use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checker, Mint , TokenAccount,TokenChecked , TokenInterface
};


pub fn token_transfer<'info>(
    from:&InterfaceAccount<'info , TokenAccount>,

    to :&InterfaceAccount<'info , TokenAccount>,
    amount :&u64,
    mint : &InterfaceAccount<'info , Mint>,
    authority :&Signer<'info>,
    
    token_program: &InterfaceAccount<'info , TokenInterface>
)->Result<()>{
 let transfer_accounts_options=TransferChecked{
  from:from.to_account_info(),
  to:to.to_account_info(),
  
  mint:mint.to_account_info(),
  
  authority:authority.to_account_info(),
 };


 let cpi_context =CpiContext::new(token_program.to_account_info(), transfer_accounts_options);

 transfer_checked(cpi_context,*amount, mint.decimals);

}