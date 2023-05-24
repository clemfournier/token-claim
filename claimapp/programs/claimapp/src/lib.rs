use anchor_lang::prelude::*;

declare_id!("79GRqHSs8MPinewuYuaSyQgTYD1WruUguy8JvRdMLe7T");

#[program]
pub mod claimapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
