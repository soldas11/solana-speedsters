use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS" ); // Placeholder Program ID

#[program]
pub mod solana_speedsters_marketplace {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";
    const MARKETPLACE_PDA_SEED: &[u8] = b"marketplace";

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        fee: u16,
    ) -> Result<()> {
        let marketplace_state = &mut ctx.accounts.marketplace_state;
        marketplace_state.authority = *ctx.accounts.authority.key;
        marketplace_state.fee = fee;
        Ok(())
    }

    pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        let listing = &mut ctx.accounts.listing;
        listing.seller = *ctx.accounts.seller.key;
        listing.mint = *ctx.accounts.nft_mint.key;
        listing.price = price;
        listing.is_listed = true;

        Ok(())
    }

    pub fn delist_nft(ctx: Context<DelistNft>) -> Result<()> {
        require!(ctx.accounts.listing.seller == ctx.accounts.seller.key(), MarketplaceError::Unauthorized);

        let (_escrow_pda, bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED, ctx.accounts.nft_mint.key().as_ref()], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], ctx.accounts.nft_mint.key().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.escrow_token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, 1)?;

        ctx.accounts.listing.is_listed = false;

        Ok(())
    }

    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        let listing = &ctx.accounts.listing;
        let marketplace_state = &ctx.accounts.marketplace_state;
        require!(listing.is_listed, MarketplaceError::NotListed);
        require!(listing.price > 0, MarketplaceError::InvalidPrice);

        let price = listing.price;
        invoke(
            &system_instruction::transfer(ctx.accounts.buyer.key, &listing.seller, price),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.seller.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let fee_amount = (price as u128 * marketplace_state.fee as u128 / 10000) as u64;
        if fee_amount > 0 {
            invoke(
                &system_instruction::transfer(ctx.accounts.buyer.key, &marketplace_state.authority, fee_amount),
                &[
                    ctx.accounts.buyer.to_account_info(),
                    ctx.accounts.marketplace_authority.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        let (_escrow_pda, bump) = Pubkey::find_program_address(&[ESCROW_PDA_SEED, ctx.accounts.nft_mint.key().as_ref()], ctx.program_id);
        let seeds = &[&ESCROW_PDA_SEED[..], ctx.accounts.nft_mint.key().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.escrow_token_account.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, 1)?;

        ctx.accounts.listing.is_listed = false;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 2,
        seeds = [MARKETPLACE_PDA_SEED],
        bump
    )]
    pub marketplace_state: Account<'info, MarketplaceState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        constraint = seller_token_account.mint == nft_mint.key(),
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.amount == 1
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = seller,
        token::mint = nft_mint,
        token::authority = escrow_token_account,
        seeds = [ESCROW_PDA_SEED, nft_mint.key().as_ref()],
        bump
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = seller,
        space = 8 + 32 + 32 + 8 + 1,
        seeds = [b"listing", nft_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

