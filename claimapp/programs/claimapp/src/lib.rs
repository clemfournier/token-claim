use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("79GRqHSs8MPinewuYuaSyQgTYD1WruUguy8JvRdMLe7T");

#[program]
pub mod claimapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, x_amount: u64, y_amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.bump = *ctx.bumps.get("escrow").unwrap();
        escrow.authority = ctx.accounts.seller.key();
        escrow.escrowed_x_tokens = ctx.accounts.escrowed_x_tokens.key();
        escrow.y_amount = y_amount; // number of token sellers wants in exchange
        escrow.y_mint = ctx.accounts.y_mint.key(); // token seller wants in exchange

        // Transfer seller's x_token in program owned escrow token account
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.seller_x_token.to_account_info(),
                    to: ctx.accounts.escrowed_x_tokens.to_account_info(),
                    authority: ctx.accounts.seller.to_account_info(),
                },
            ),
            x_amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    /// `seller`, who is willing to sell his token_x for token_y
    #[account(mut)]
    seller: Signer<'info>,

    /// Token x mint for ex. USDC
    x_mint: Account<'info, Mint>,
    /// Token y mint 
    y_mint: Account<'info, Mint>,

    /// ATA of x_mint 
    #[account(mut, constraint = seller_x_token.mint == x_mint.key() && seller_x_token.owner == seller.key())] 
    seller_x_token: Account<'info, TokenAccount>,

    #[account(
        init, 
        payer = seller,  
        space=Escrow::LEN,
        seeds = ["escrow6".as_bytes(), seller.key().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        init,
        payer = seller,
        token::mint = x_mint,
        token::authority = escrow,
    )]
    escrowed_x_tokens: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Escrow {
    authority: Pubkey,
    bump: u8,
    escrowed_x_tokens: Pubkey,
    y_mint: Pubkey,
    y_amount: u64,
}

impl Escrow {
    pub const LEN: usize = 8 + 1+ 32 + 32 + 32 + 8;
}