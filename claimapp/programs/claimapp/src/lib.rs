use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_lang::system_program;
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("6YF6WkHwsNwssuXWBi1BktqgpC27QyoJw9cd3VDrobZi");
// declare_id!("Bh3kNWhE4PbiSAzcCNRfbPPfewNk5y6HFxWhYMRok7xm");

#[program]
pub mod claimapp {
    use super::*;

    pub const CLAIM_AMOUNT: u64 = 1;
    pub const TREASURY: &[u8] = b"treasury8";
    pub const CONTRACT: &[u8] = b"contract8";
    pub const CLAIM: &[u8] = b"claim8";
    pub const TOKEN_MINT: &Pubkey = &pubkey!("CCoin6VDphET1YsAgTGsXwThEUWetGNo4WiTPhGgR6US");
    pub const NFT_UPDATE_AUTHORITY: &Pubkey = &pubkey!("En54STTsmVrWA3Cd43SQNgiLrihRDG2iMJD6zWPHjYfW");
    pub const OWNERS: &[Pubkey] = &[
        pubkey!("EjvRc5HRynCfZu74QUDMs5iunHcKiSsyuKUxuNdgMFzz"),
        pubkey!("FZ5FgLRom1Xv9dUGxTTJX5tU5We6BgyWXw3GytWaU7op")
    ];

    pub fn init_contract(ctx: Context<InitContract>, limit: u64) -> Result<()> {
        // NICE TO HAVE (TO BE ABLE TO SHOW A NICE ERROR MESSAGE):
        //// CHECK IF ENOUGH SOL TO CREATE THE CONTRACT

        ctx.accounts.claim_contract.bump = *ctx.bumps.get("claim_contract").unwrap();
        ctx.accounts.claim_contract.is_active = true;
        ctx.accounts.claim_contract.limit = limit;
        ctx.accounts.claim_contract.claimed = 0;

        msg!("Created a new claim contract, limit {0} claims", limit);

        Ok(())
    }

    pub fn init_treasury(ctx: Context<InitTreasury>, amount: u64) -> Result<()> {
        // NICE TO HAVE (TO BE ABLE TO SHOW A NICE ERROR MESSAGE):
        //// CHECK IF ENOUGH SOL TO CREATE THE TREASURY
        //// CHECK IF THE DEPOSITOR HAS ENOUGH TOKENS TO DEPOSIT 

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

    pub fn add_to_treasury(ctx: Context<AddToTreasury>, amount: u64, amount_sol: u64) -> Result<()> {
        // NICE TO HAVE (TO BE ABLE TO SHOW A NICE ERROR MESSAGE):
        //// CHECK IF ENOUGH SOL TO DEPOSIT
        //// CHECK IF THE DEPOSITOR HAS ENOUGH TOKENS TO DEPOSIT
        
        if amount > 0 {
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
        }

        if amount_sol > 0 {
            system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    system_program::Transfer {
                        from: ctx.accounts.depositor.to_account_info(),
                        to: ctx.accounts.treasury.to_account_info(),
                        //to: ctx.accounts.sol_treasury.to_account_info(),
                    }),
                    amount_sol,
            )?;
        }

        msg!("Added {0} token and {1} SOL", amount, amount_sol);

        Ok(())
    }

    pub fn init_claim(ctx: Context<InitClaim>) -> Result<()> {
        // WAY MORE TEST
        // CHECK IF THE CLAIMER IS THE OWNER OF THE NFT
        // CHECK IF THE CLAIMER DIDNT ALREADY CLAIMED
        // CHECK IF DIDNT REACH THE MAX CLAIMERS
    
        // CREATING THE CLAIM TOKEN ACCOUNT
        let claim_account_data = &mut ctx.accounts.claim_account;
        claim_account_data.bump = *ctx.bumps.get("claim_account").unwrap();
        claim_account_data.amount = CLAIM_AMOUNT;
        claim_account_data.owner = ctx.accounts.signer.key();
        claim_account_data.mint = ctx.accounts.mint.key();

        // TRANSFER TOKENS TO THE CLAIM ACCOUNT
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.treasury_token_account.to_account_info(),
                    to: ctx.accounts.claimer_token_account.to_account_info(),
                    authority: ctx.accounts.treasury.to_account_info(), 
                },
                &[&[TREASURY.as_ref(), &[ctx.accounts.treasury.bump]]],
            ),
            ctx.accounts.treasury_token_account.amount,
        )?;

        // TRANSFER SOL TO THE OWNER TO PAY FOR THE TOKEN ACCOUNT(S) CREATION
        // CHECK IF BONK TOKEN ACCOUNT AND CLAIM TOKEN ACCOUNT NEED TO BE CREATED
        let pda_cost: u64  = 1454640; // COST FOR CREATING PDA TO STORE CLAIM STATUS
        let token_account_cost: u64 = 200000; // COST FOR CREATING THE CLAIMED TOKEN, TOKEN ACCOUNT

        let vault_account_info: &mut AccountInfo = &mut ctx.accounts.treasury.to_account_info();
        //let vault_account_info: &mut AccountInfo = &mut ctx.accounts.sol_treasury.to_account_info();
        let owner_account_info: &mut AccountInfo = &mut ctx.accounts.signer.to_account_info();

        // MAKE VERIFICATION WITH THAT 
        // let vault_lamports_initial = vault_account_info.lamports();
        // let owner_lamports_initial = owner_account_info.lamports();

        **owner_account_info.lamports.borrow_mut() += pda_cost;
        **vault_account_info.lamports.borrow_mut() -= pda_cost;

        // system_program::transfer(
        //     CpiContext::new(
        //         ctx.accounts.system_program.to_account_info(),
        //         system_program::Transfer {
        //             from: ctx.accounts.sol_treasury.to_account_info(),
        //             to: ctx.accounts.signer.to_account_info(),
        //         }),
        //     pda_cost,
        // )?;

        // UPDATE CONTRACT DATA
        // CHECK IF WE REACHED THE LIMIT
        ctx.accounts.claim_contract.claimed += 1;

        msg!("{0} claimed {1} tokens, for NFT {2}. {3} people have now claimed for {4} max", 
            ctx.accounts.signer.key(),
            CLAIM_AMOUNT,
            ctx.accounts.mint.key(),
            ctx.accounts.claim_contract.claimed, 
            ctx.accounts.claim_contract.limit
        );

        msg!("Created a new claim account {0}, mint {1} owner {2} amount {3}", 
            ctx.accounts.claim_account.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.signer.key(),
            CLAIM_AMOUNT
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
    #[account(mut, constraint = OWNERS.contains(&depositor.key()))]
    depositor: Signer<'info>,

    /// Token mint
    #[account(constraint = TOKEN_MINT.key() == mint.key())] 
    mint: Account<'info, Mint>,

    /// ATA of x_mint 
    #[account(mut, constraint = depositor_token_account.mint == mint.key() && depositor_token_account.owner == depositor.key())] 
    depositor_token_account: Account<'info, TokenAccount>,

    #[account(
        init, 
        payer = depositor,  
        space=Treasury::LEN,
        seeds = [TREASURY.as_ref()],
        bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        init, 
        payer = depositor,  
        space=48
    )]
    // SEE WHY I HAVE TO PUT "CHECK:"
    /// CHECK:
    pub sol_treasury: AccountInfo<'info>,

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
pub struct AddToTreasury<'info> {

    /// Deposit authority, owner only
    #[account(mut, constraint = OWNERS.contains(&depositor.key()))]
    depositor: Signer<'info>,

    /// Token mint
    mint: Account<'info, Mint>,

    /// ATA of x_mint 
    #[account(mut, constraint = depositor_token_account.mint == mint.key() && depositor_token_account.owner == depositor.key())] 
    depositor_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [TREASURY.as_ref()],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,

    #[account(
        mut,
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
    // Global account to store the claims
    #[account(
        init, 
        payer = signer,
        space = Contract::LEN,
        seeds = [CONTRACT.as_ref()],
        bump,
    )] 
    pub claim_contract: Account<'info, Contract>,

    // Signer, has to be an owner
    #[account(mut, constraint = OWNERS.contains(&signer.key()))]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitClaim<'info> {
    // Signer, person who wants to claim the tokens
    #[account(mut)]
    pub signer: Signer<'info>,

    // Claim account, PDA to store the claim information 
    #[account(
        init, 
        payer = signer, 
        space = Claim::LEN,
        seeds = [CLAIM.as_ref(), signer.key().as_ref()],
        bump,
    )] 
    pub claim_account: Account<'info, Claim>,

    // Treasury account, account who hold the token's token account
    #[account(
        mut,
        seeds = [TREASURY.as_ref()],
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
        mut,
        seeds = [CONTRACT.as_ref()],
        bump = claim_contract.bump,
    )] 
    pub claim_contract: Account<'info, Contract>,

    #[account(
        mut,
        constraint = mint.key() == nft_token_account.mint,
        // OWNER VERIFICATION, REMOVE LATER
        // constraint = nft_token_account.owner == signer.key(),
        constraint = nft_token_account.amount == 1,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    // NFT mint of the owner
    // Might have some more verifications here
    #[account(
        token::authority = NFT_UPDATE_AUTHORITY
    )]
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
    pub bump: u8,
    pub is_active: bool,
    pub claimed: u64,
    pub limit: u64
}

impl Contract {
    const LEN: usize = 
        8 + // discriminator
        1 + // bump
        1 + // bool
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
