use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("Econ111111111111111111111111111111111111"); // Placeholder Program ID

#[program]
pub mod economy {
    use super::*;

    pub fn create_vesting_schedule(
        ctx: Context<CreateVestingSchedule>,
        beneficiary: Pubkey,
        mint: Pubkey,
        total_amount: u64,
        start_ts: i64,
        cliff_ts: i64,
        end_ts: i64,
    ) -> Result<()> {
        let vesting_schedule = &mut ctx.accounts.vesting_schedule;
        vesting_schedule.authority = *ctx.accounts.authority.key;
        vesting_schedule.beneficiary = beneficiary;
        vesting_schedule.mint = mint;
        vesting_schedule.total_amount = total_amount;
        vesting_schedule.start_ts = start_ts;
        vesting_schedule.cliff_ts = cliff_ts;
        vesting_schedule.end_ts = end_ts;
        vesting_schedule.released_amount = 0;

        let cpi_accounts = Transfer {
            from: ctx.accounts.authority_token_account.to_account_info(),
            to: ctx.accounts.vesting_vault.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, total_amount)?;

        Ok(())
    }

    pub fn release_vested_tokens(ctx: Context<ReleaseVestedTokens>) -> Result<()> {
        let clock = Clock::get()?;
        let current_ts = clock.unix_timestamp;
        let schedule = &mut ctx.accounts.vesting_schedule;

        let vested_amount = calculate_vested_amount(
            current_ts,
            schedule.start_ts,
            schedule.cliff_ts,
            schedule.end_ts,
            schedule.total_amount,
        );

        require!(vested_amount > schedule.released_amount, EconomyError::NoTokensToRelease);

        let amount_to_release = vested_amount.checked_sub(schedule.released_amount).unwrap();

        let (_vault_pda, bump) = Pubkey::find_program_address(&[b"vesting-vault", schedule.key().as_ref()], ctx.program_id);
        let seeds = &[&b"vesting-vault"[..], schedule.key().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vesting_vault.to_account_info(),
            to: ctx.accounts.beneficiary_token_account.to_account_info(),
            authority: ctx.accounts.vesting_vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount_to_release)?;

        schedule.released_amount = schedule.released_amount.checked_add(amount_to_release).unwrap();

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.staking_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts), amount)?;

        let stake_account = &mut ctx.accounts.stake_account;
        stake_account.owner = *ctx.accounts.user.key;
        stake_account.balance = stake_account.balance.checked_add(amount).unwrap();
        stake_account.last_staked_ts = Clock::get()?.unix_timestamp;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let stake_account = &mut ctx.accounts.stake_account;
        require!(stake_account.balance >= amount, EconomyError::InsufficientStake);

        let (_vault_pda, bump) = Pubkey::find_program_address(&[b"staking-vault", ctx.accounts.stake_mint.key().as_ref()], ctx.program_id);
        let seeds = &[&b"staking-vault"[..], ctx.accounts.stake_mint.key().as_ref(), &[bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.staking_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.staking_vault.to_account_info(),
        };
        token::transfer(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer), amount)?;

        stake_account.balance = stake_account.balance.checked_sub(amount).unwrap();

        Ok(())
    }
}

fn calculate_vested_amount(current_ts: i64, start_ts: i64, cliff_ts: i64, end_ts: i64, total_amount: u64) -> u64 {
    if current_ts < cliff_ts {
        return 0;
    }
    if current_ts >= end_ts {
        return total_amount;
    }

    let duration = end_ts.checked_sub(start_ts).unwrap();
    let elapsed = current_ts.checked_sub(start_ts).unwrap();

    (total_amount as u128)
        .checked_mul(elapsed as u128).unwrap()
        .checked_div(duration as u128).unwrap() as u64
}

#[derive(Accounts)]
pub struct CreateVestingSchedule<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8
    )]
    pub vesting_schedule: Account<'info, VestingSchedule>,
    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = vesting_vault,
        seeds = [b"vesting-vault", vesting_schedule.key().as_ref()],
        bump
    )]
    pub vesting_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

