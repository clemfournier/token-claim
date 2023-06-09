use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_lang::system_program;
use solana_program::{pubkey, pubkey::Pubkey};
use mpl_token_metadata;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};
use base64;
declare_id!("FReLJ3SZ6CAMg7QszqukqqtXC4yYdPv4pcu6ARajVAoG");

#[program]
pub mod claimapp {
    use super::*;

    pub const TREASURY: &[u8] = b"treasury9";
    pub const CONTRACT: &[u8] = b"contract9";
    pub const CLAIM: &[u8] = b"claim9";

    pub const OWNERS: &[Pubkey] = &[
        pubkey!("EjvRc5HRynCfZu74QUDMs5iunHcKiSsyuKUxuNdgMFzz"),
        pubkey!("FZ5FgLRom1Xv9dUGxTTJX5tU5We6BgyWXw3GytWaU7op")
    ];

    pub fn init_contract(ctx: Context<InitContract>, limit: u64, collection_name: String, claim_amount: u64) -> Result<()> {
        // NICE TO HAVE (TO BE ABLE TO SHOW A NICE ERROR MESSAGE):
        //// CHECK IF ENOUGH SOL TO CREATE THE CONTRACT

        ctx.accounts.claim_contract.bump = *ctx.bumps.get("claim_contract").unwrap();
        ctx.accounts.claim_contract.is_active = true;
        ctx.accounts.claim_contract.limit = limit;
        ctx.accounts.claim_contract.claimed = 0;
        ctx.accounts.claim_contract.claim_amount = claim_amount;
        ctx.accounts.claim_contract.collection_name = collection_name;
        ctx.accounts.claim_contract.mint = ctx.accounts.mint.key();
        ctx.accounts.claim_contract.update_authority = ctx.accounts.update_authority.key();


        msg!("Created a new claim contract, limit {0} claims", limit);

        Ok(())
    }

    pub fn update_contract(ctx: Context<UpdateContract>, limit: u64, collection_name: String, claim_amount: u64, is_active: bool) -> Result<()> {
        // NICE TO HAVE (TO BE ABLE TO SHOW A NICE ERROR MESSAGE):
        //// CHECK IF LIMIT UNDER THE CLAIMED AMOUNT? 

        ctx.accounts.claim_contract.is_active = is_active;
        ctx.accounts.claim_contract.limit = limit;
        ctx.accounts.claim_contract.claim_amount = claim_amount;
        ctx.accounts.claim_contract.collection_name = collection_name;
        ctx.accounts.claim_contract.update_authority = ctx.accounts.update_authority.key();

        msg!("Updated claim contract");

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

    pub fn init_claim(ctx: Context<InitClaim>, new_token_account: bool) -> Result<()> {
        // WAY MORE TEST
        // CHECK IF THE CLAIMER DIDNT ALREADY CLAIMED (SHOULD BE OK BECAUSE CANNOT CREATE THE PDA TWICE)
        // CHECK IF DIDNT REACH THE MAX CLAIMERS
        // CHECK IF THE SOL VAULT HAS ENOUGH TOKENS (SHOULD FAIL BY ITSELF) -> DONE
        // CHECK IF THE TOKEN VAULT HAS ENOUGH TOKENS (SHOULD FAIL BY ITSELF)

        // MIGHT BE USEFUL CODE TO VERIFY DEEPER NFT 
        // let (metadata, _) = Pubkey::find_program_address(
        //     &[
        //         mpl_token_metadata::state::PREFIX.as_bytes(),
        //         mpl_token_metadata::id().as_ref(),
        //         ctx.accounts.nft_token_account.mint.key().as_ref(),
        //         // ctx.accounts.mint.key().as_ref(),
        //     ],
        //     &mpl_token_metadata::id(),
        // );
        // if mint_metadata.collection.is_some() {
        //     let collection = mint_metadata.collection.unwrap();
        //     if collection.verified  {
        //         msg!("Collection verified {0}", collection.key);
        //         msg!("Metadata update auth: {0}", mint_metadata.update_authority.key());
        //     }
        // } 

        // VERIFY THE NFT
        let mint_metadata= Metadata::from_account_info(&ctx.accounts.nft_metadata.to_account_info())?;

        if ctx.accounts.nft_token_account.mint.key() != mint_metadata.mint.key() {
            msg!("Mismatch mint (retrived: {0}, metadata sent: {1})", mint_metadata.mint.key(), ctx.accounts.nft_token_account.mint.key());
            return err!(CustomErrorCode::MetadataMismatch);
        }

        if mint_metadata.update_authority.key() != ctx.accounts.claim_contract.update_authority.key() {
            msg!("Mismatch update authority (retrived: {0}, expected: {1})", mint_metadata.update_authority.key(), ctx.accounts.claim_contract.update_authority.key());
            return err!(CustomErrorCode::UpdateAuthorityMismatch);
        }

        let nft_symbol = ctx.accounts.claim_contract.collection_name.to_string();
        let metadata_symbol = base64::encode(mint_metadata.data.symbol);

        if metadata_symbol != nft_symbol {
            msg!("Mismatch name (retrived: {0}, expected: {1})", metadata_symbol, nft_symbol);
            return err!(CustomErrorCode::SymbolMismatch);
        }
    
        // CREATING THE CLAIM TOKEN ACCOUNT
        let claim_account_data = &mut ctx.accounts.claim_account;
        claim_account_data.bump = *ctx.bumps.get("claim_account").unwrap();
        claim_account_data.amount = ctx.accounts.claim_contract.claim_amount;
        claim_account_data.owner = ctx.accounts.signer.key();
        claim_account_data.mint = ctx.accounts.nft_token_account.mint.key();

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
            ctx.accounts.claim_contract.claim_amount,
        )?;

        // TRANSFER SOL TO THE OWNER TO PAY FOR THE TOKEN ACCOUNT(S) CREATION
        let pda_cost: u64  = 1454640;
        let token_account_cost: u64 = 2039280;

        let mut total_cost: u64 = pda_cost;

        // RISK: USER CAN MANUALLY PUT TRUE AND GET THE SOL TOKEN ACCOUNT FOR FREE
        //       BUT SINCE THE CLAIM CAN'T BE CALLED TWICE, IT'S NOT A BIG DEAL 
        if new_token_account == true {
            total_cost += token_account_cost;
        }

        let vault_account_info: &mut AccountInfo = &mut ctx.accounts.treasury.to_account_info();
        let owner_account_info: &mut AccountInfo = &mut ctx.accounts.signer.to_account_info();

        let vault_lamports_initial = vault_account_info.lamports();

        if vault_lamports_initial < total_cost {
            return err!(CustomErrorCode::VaultDoesntHaveEnoughSol);
        }

        **owner_account_info.lamports.borrow_mut() += total_cost;
        **vault_account_info.lamports.borrow_mut() -= total_cost;

        ctx.accounts.claim_contract.claimed += 1;

        msg!("{0} claimed {1} tokens, for NFT {2}. {3}/{4} claimed", 
            ctx.accounts.signer.key(),
            ctx.accounts.claim_contract.claim_amount,
            ctx.accounts.nft_token_account.mint.key(),
            ctx.accounts.claim_contract.claimed, 
            ctx.accounts.claim_contract.limit
        );
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitTreasury<'info> {

    /// Deposit authority
    #[account(mut, constraint = OWNERS.contains(&depositor.key()))]
    depositor: Signer<'info>,

    /// Token mint
    #[account(constraint = claim_contract.mint.key() == mint.key())] 
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

    // Claim contract account, global account for storing claim counts
    #[account(
        mut,
        seeds = [CONTRACT.as_ref()],
        bump = claim_contract.bump,
    )] 
    pub claim_contract: Account<'info, Contract>,

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

    /// Token mint
    mint: Account<'info, Mint>,

    /// Update Authority
    /// CHECK: This is fine
    update_authority: AccountInfo<'info>,

    // Signer, has to be an owner
    #[account(mut, constraint = OWNERS.contains(&signer.key()))]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateContract<'info> {
    // Global account to store the claims
    #[account(
        mut,
        seeds = [CONTRACT.as_ref()],
        bump = claim_contract.bump,
    )] 
    pub claim_contract: Account<'info, Contract>,

    /// Update Authority
    /// CHECK: This is fine
    update_authority: AccountInfo<'info>,

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
    #[account(
        mut,
        constraint = treasury_token_account.owner == treasury.key(),
        constraint = treasury_token_account.mint == claim_contract.mint.key(),
    )]
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
        // TODO: OWNER VERIFICATION, UNCOMMENT LATER
        // constraint = nft_token_account.owner == signer.key(),
        constraint = nft_token_account.amount == 1,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because the nft_metadata is not mutable
    pub nft_metadata: AccountInfo<'info>,

    token_program: Program<'info, Token>,
    // rent: Sysvar<'info, Rent>,
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
    pub claimed: u64,
    pub limit: u64,
    pub bump: u8,
    pub mint: Pubkey,
    pub update_authority: Pubkey,
    pub collection_name: String,
    pub claim_amount: u64,
}

impl Contract {
    const LEN: usize = 
        8 + // discriminator
        1 + // bool
        8 + // u64
        8 + // u64
        1 + // bump
        32 + // Pubkey
        32 + // Pubkey
        24 + // String
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

#[error_code]
pub enum CustomErrorCode {
    #[msg("NFT Mismatch metadata")]
    MetadataMismatch,
    #[msg("NFT Mismatch update authority")]
    UpdateAuthorityMismatch,
    #[msg("NFT Mismatch symbol")]
    SymbolMismatch,
    #[msg("Vault doesnt have enough SOL")]
    VaultDoesntHaveEnoughSol,
}