pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BcsDNQr788YYTe9D2UsGTzAqEJHL5Xu1zupogfZ38szp");

#[program]
pub mod swap_contract {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        initialize::make_offer::send_offered_tokens_to_vault()?;
        initialize::make_offer::save_offer();
    }
}
