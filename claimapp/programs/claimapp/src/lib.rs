use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("79GRqHSs8MPinewuYuaSyQgTYD1WruUguy8JvRdMLe7T");

#[program]
pub mod claimapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let seeds: &[&[&[u8]]] = &[&[&[b'p', b'r', b'e', b'f', b'i', b'x'], &[1u8]]];

        let ix = spl_token::instruction::initialize_account(
            &spl_token::ID,
            &ctx.accounts.bonk_escrow.to_account_info().key,
            &ctx.accounts.bonk_mint.to_account_info().key,
            &ctx.accounts.authority.to_account_info().key,
        )?;

        invoke_signed(&ix, &[
            ctx.accounts.bonk_escrow.clone(),
            ctx.accounts.bonk_mint.clone(),
            ctx.accounts.authority.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
        ], seeds);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(signer)]
    authority: AccountInfo<'info>,
    bonk_escrow: AccountInfo<'info>,
    bonk_mint: AccountInfo<'info>,
    #[account("spl_token::ID")]
    token_program: AccountInfo<'info>,
    #[account("solana_system_program::ID")]
    system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(signer)]
    authority: AccountInfo<'info>,
    bonk_escrow: AccountInfo<'info>,
    to: AccountInfo<'info>,
    #[account("spl_token::ID")]
    token_program: AccountInfo<'info>,
}