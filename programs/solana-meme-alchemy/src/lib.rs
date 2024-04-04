use anchor_lang::prelude::*;

declare_id!("GCmY5vbNkFNdM5ofYcNHnJ8ivZNzHLVP3vEx6vUbYdFQ");

#[program]
pub mod solana_meme_alchemy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
