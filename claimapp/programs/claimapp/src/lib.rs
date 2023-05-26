use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// declare_id!("79GRqHSs8MPinewuYuaSyQgTYD1WruUguy8JvRdMLe7T");
declare_id!("Bh3kNWhE4PbiSAzcCNRfbPPfewNk5y6HFxWhYMRok7xm");

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

    pub fn init_claim(ctx: Context<InitClaim>) -> Result<()> {
        // AMOUNT WITH THE DECIMALS
        let amount = 1;
    
        // WAY MORE TEST
        // CHECK IF THE CLAIMER IS THE OWNER OF THE NFT
        // CHECK IF THE CLAIMER DIDNT ALREADY CLAIMED
    
        // CREATING THE CLAIM TOKEN ACCOUNT
        let claim_account_data = &mut ctx.accounts.claim_account;
        claim_account_data.bump = *ctx.bumps.get("claim_account").unwrap();
        claim_account_data.amount = amount;
        claim_account_data.owner = ctx.accounts.signer.key();
        claim_account_data.mint = ctx.accounts.mint.key();
    
        msg!("Created a new claim account {0}, mint {1} owner {2} amount {3}", 
            ctx.accounts.claim_account.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.signer.key(),
            amount
        );
    
        // TRANSFER SOL TO THE OWNER TO PAY FOR THE TOKEN ACCOUNT(S) CREATION
        // CHECK IF BONK TOKEN ACCOUNT AND CLAIM TOKEN ACCOUNT NEED TO BE CREATED
    
        // TRANSFER TOKENS TO THE CLAIM ACCOUNT
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.claimer_token_account.to_account_info(),
                    authority: ctx.accounts.treasury.to_account_info(), 
                },
                &[&["treasury6".as_bytes(), ctx.accounts.depositor.key().as_ref(), &[ctx.accounts.treasury.bump]]],
            ),
            ctx.accounts.treasury_token_account.amount,
        )?;

        // UPDATE CONTRACT DATA
        // CHECK IF WE REACHED THE LIMIT
        let claim_account_data = &mut ctx.accounts.claim_contract_account;
        claim_account_data.claimed += 1;

        msg!("{0} claimed {1} tokens, for NFT {2}. {3} people have now claimed for {4} max", 
            ctx.accounts.signer.key(),
            amount,
            ctx.accounts.mint.key(),
            claim_account_data.claimed, 
            claim_account_data.limit
        );
    
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
    // Signer, person who wants to claim the tokens
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub depositor: AccountInfo<'info>,

    // Claim account, PDA to store the claim information 
    #[account(
        init, 
        payer = signer, 
        space = Claim::LEN,
        seeds = [b"claim".as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub claim_account: Account<'info, Claim>,

    // Treasury account, account who hold the token's token account
    #[account(
        mut,
        seeds = ["treasury6".as_bytes(), depositor.key().as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    // Treasury token account, token account who hold the tokens
    #[account(mut, constraint = treasury_token_account.key() == treasury.treasury_token_account)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    // Signer/Claimer token account, token account who will receive the tokens
    #[account(
        mut,
        constraint = claimer_token_account.mint == treasury_token_account.mint,
        constraint = claimer_token_account.owner == signer.key()
    )]
    claimer_token_account: Account<'info, TokenAccount>,

    // Claim contract account, global account for storing claim counts
    #[account(
        seeds = [b"claimcontract".as_ref(), depositor.key().as_ref()],
        bump = claim_contract_account.bump,
    )] 
    pub claim_contract_account: Account<'info, Contract>,

    // NFT mint of the owner
    // Might have some more verifications here
    pub mint: Account<'info, Mint>,

    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}

#[account]
pub struct ClaimTreasury {
    depositor: Pubkey,
    treasury_token_account: Pubkey,
    pub bump: u8,
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
    pub bump: u8,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub amount: u64
}

impl Claim {
    const LEN: usize = 
        8 + // discriminator
        1 + // bump
        32 + // Pubkey
        32 + // Pubkey
        8; // u64
}
