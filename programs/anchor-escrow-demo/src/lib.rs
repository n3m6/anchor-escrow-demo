pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::{declare_id, program, Accounts, Context};

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GXRhfqiKXbyePDMvU2TAre2RWFAuwVDCAJ7J55eCU8yZ");

#[program]
pub mod anchor_escrow_demo {
    use super::*;


    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)
    }
}
