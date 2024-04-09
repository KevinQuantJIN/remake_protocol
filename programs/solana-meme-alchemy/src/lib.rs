
pub mod instructions;
pub mod state;
pub mod utils;
pub mod errors;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("2wMP4GLFkKV3eZnr17PnB4JStRzUN4oet4xmvmgHWq9t");

#[program]
pub mod hedge_take_home {
    use super::*;

    pub fn init_pool(ctx: Context<InitializePool>) -> Result<()> {
        init_pool::handler(ctx)
    }

    pub fn init_stake_entry(ctx: Context<InitEntryCtx>) -> Result<()> {
        init_stake_entry::handler(ctx)
    }

    pub fn stake(ctx: Context<StakeCtx>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

}