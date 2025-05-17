// Import Anchor's prelude — this includes everything you need to build a smart contract on Solana using the Anchor framework.
use anchor_lang::prelude::*;

// Import key components from the SPL Token Interface package provided by Anchor:
// - `transfer_checked`: a safe token transfer function that validates mint decimals
// - `Mint`: represents the token mint account (defines the token type)
// - `TokenAccount`: represents a user's token wallet
// - `TokenInterface`: the program we call to handle token logic
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenChecked, TokenInterface,
};

// Define a public function called `token_transfer` that returns a Result (success/failure)
pub fn token_transfer<'info>(
    // `from` is the sender’s token account (where tokens will be transferred from)
    from: &InterfaceAccount<'info, TokenAccount>,

    // `to` is the recipient’s token account (where tokens will go)
    to: &InterfaceAccount<'info, TokenAccount>,

    // `amount` is how many tokens to transfer (in smallest units, like lamports or wei)
    amount: &u64,

    // `mint` is the mint account associated with this token (helps validate token type and decimals)
    mint: &InterfaceAccount<'info, Mint>,

    // `authority` is the signer (must own the `from` account or be a delegate)
    authority: &Signer<'info>,

    // `token_program` is the interface to the SPL Token Program (this performs the transfer logic)
    token_program: &Interface<'info, TokenInterface>,
) -> Result<()> {
    // STEP 1: Prepare the required accounts for the `transfer_checked` instruction.
    // This struct defines the roles and addresses involved in the transfer.
    let transfer_accounts_options = TransferChecked {
        from: from.to_account_info(),         // Convert `from` account to AccountInfo type for CPI
        to: to.to_account_info(),             // Convert `to` account to AccountInfo type for CPI
        mint: mint.to_account_info(),         // Provide mint account (token definition)
        authority: authority.to_account_info(), // Authority who will sign the transfer
    };

    // STEP 2: Create a CPI (Cross Program Invocation) context
    // This lets us call the SPL Token Program securely from within our program
    let cpi_context = CpiContext::new(
        token_program.to_account_info(),      // The token program we’re calling (usually the SPL Token Program)
        transfer_accounts_options             // Accounts needed to execute the transfer
    );

    // STEP 3: Execute the token transfer using the `transfer_checked` function
    // This version ensures:
    // - The mint provided is correct
    // - The number of decimals matches the token standard
    // It avoids unsafe transfers due to mismatched token metadata
    transfer_checked(
        cpi_context,                          // Context containing all the required accounts and token program
        *amount,                              // The number of tokens to send (dereferenced from pointer)
        mint.decimals                         // The number of decimals defined for this token (used for accuracy)
    )
}
