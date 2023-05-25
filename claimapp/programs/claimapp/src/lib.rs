use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// declare_id!("79GRqHSs8MPinewuYuaSyQgTYD1WruUguy8JvRdMLe7T");
declare_id!("7tnWDxyukYms6pdd2hj4sFV7VBMsFNWqfMUKkpcDYSap");

#[program]
pub mod claimapp {
    use super::*;

    pub fn init_contract(ctx: Context<InitContract>, limit: u64) -> Result<()> {
        let claim_account_data = &mut ctx.accounts.claim_contract_account;
        claim_account_data.bump = *ctx.bumps.get("claim_contract_account").unwrap();

        claim_account_data.is_active = true;
        claim_account_data.limit = limit;
        claim_account_data.claimed = 0;

        msg!("Created a new claim contract account, limit {0}", limit);

        Ok(())
    }

    pub fn init_claim(ctx: Context<InitClaim>) -> Result<()> {
        let claim_account_data = &mut ctx.accounts.claim_account;
        claim_account_data.bump = *ctx.bumps.get("claim_account").unwrap();

        claim_account_data.is_claimed = false;

        msg!("Created a new claim account {0}", ctx.accounts.claim_account.key());

        Ok(())
    }

    pub fn init_treasury(ctx: Context<InitTreasury>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.treasury;
        escrow.bump = *ctx.bumps.get("treasury").unwrap();
        escrow.depositor = ctx.accounts.depositor.key();
        escrow.treasury_token_account = ctx.accounts.treasury_token_account.key();

        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.depositor_token_account.to_account_info(),
                    to: ctx.accounts.treasury_token_account.to_account_info(),
                    authority: ctx.accounts.depositor.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!("Created a new treasury with {0} token", amount);

        Ok(())
    }

    pub fn claim_token(ctx: Context<ClaimToken>) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.escrowed_x_tokens.to_account_info(),
                    to: ctx.accounts.claimer_x_token.to_account_info(),
                    authority: ctx.accounts.escrow.to_account_info(), 
                },
                &[&["escrow6".as_bytes(), ctx.accounts.seller.key().as_ref(), &[ctx.accounts.escrow.bump]]],
            ),
            ctx.accounts.escrowed_x_tokens.amount,
        )?;

        Ok(())
    }

        // anchor_spl::token::close_account(CpiContext::new_with_signer(
        //     ctx.accounts.token_program.to_account_info(),
        //     anchor_spl::token::CloseAccount {
        //         account: ctx.accounts.escrowed_x_tokens.to_account_info(),
        //         destination: ctx.accounts.seller.to_account_info(),
        //         authority: ctx.accounts.escrow.to_account_info(),
        //     },
        //     &[&["escrow6".as_bytes(), ctx.accounts.seller.key().as_ref(), &[ctx.accounts.escrow.bump]]],
        // ))?;

}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    pub claimer: Signer<'info>,

    pub seller: AccountInfo<'info>,

    #[account(
        mut,
        close = seller, constraint = escrow.depositor == seller.key(),
        seeds = ["escrow6".as_bytes(), escrow.depositor.as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Treasury>,

    #[account(mut, constraint = escrowed_x_tokens.key() == escrow.treasury_token_account)]
    pub escrowed_x_tokens: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = claimer_x_token.mint == escrowed_x_tokens.mint,
        constraint = claimer_x_token.owner == claimer.key()
    )]
    claimer_x_token: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {

    /// Deposit authority
    /// TODO: Check if it's the authorized account
    #[account(mut)]
    depositor: Signer<'info>,

    /// Token mint
    mint: Account<'info, Mint>,

    /// ATA of x_mint 
    #[account(mut, constraint = depositor_token_account.mint == mint.key() && depositor_token_account.owner == depositor.key())] 
    depositor_token_account: Account<'info, TokenAccount>,

    #[account(
        init, 
        payer = depositor,  
        space=Treasury::LEN,
        seeds = ["treasury6".as_bytes(), depositor.key().as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init,
        payer = depositor,
        token::mint = mint,
        token::authority = treasury,
    )]
    treasury_token_account: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitContract<'info> {
    // Making a global account for storing votes
    #[account(
        init, 
        payer = signer, 
        space = Contract::LEN,
        seeds = [b"claimcontract".as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub claim_contract_account: Account<'info, Contract>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitClaim<'info> {
    // Making a global account for storing votes
    #[account(
        init, 
        payer = signer, 
        space = Claim::LEN,
        seeds = [b"claim".as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub claim_account: Account<'info, Claim>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Treasury {
    depositor: Pubkey,
    bump: u8,
    treasury_token_account: Pubkey,
    deposited: u64
}

impl Treasury {
    const LEN: usize = 
        8 + // discriminator
        32 + // pubkey
        1 + // bump
        32 + // pubkey
        8; // u64
}

#[account]
#[derive(Default)]
pub struct Contract {
    pub is_active: bool,
    pub bump: u8,
    pub claimed: u64,
    pub limit: u64
}

impl Contract {
    const LEN: usize = 
        8 + // discriminator
        1 + // bool
        1 + // bump
        8 + // u64
        8; // u64
}

#[account]
#[derive(Default)]
pub struct Claim {
    pub is_claimed: bool,
    pub bump: u8,
    pub owner: Pubkey,
    pub mint: Pubkey,
}

impl Claim {
    const LEN: usize = 
        8 + // discriminator
        1 + // bool
        1 + // bump
        32 + // Pubkey
        32; // Pubkey
}