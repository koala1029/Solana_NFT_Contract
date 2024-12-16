use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::models::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, _bumps: &WithdrawBumps, amount: u64) -> Result<()> {
        let config = &mut self.config;
        let user = &mut self.user;
        require!(
            config.primary_treasury == user.key() || config.secondary_treasury == user.key(),
            ErrorCode::UnauthorizedCaller
        );
        if config.primary_treasury == user.key() {
            require!(
                config.primary_balance >= amount,
                ErrorCode::InsufficientAmount
            );
            config.primary_balance -= amount;
        } else if config.secondary_treasury == user.key() {
            require!(
                config.secondary_balance >= amount,
                ErrorCode::InsufficientAmount
            );
            config.secondary_balance -= amount;
        }

        **self.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **self.user.to_account_info().try_borrow_mut_lamports()? += amount;
        msg!("Withdraw: {} -> {}", self.user.key(), amount);
        Ok(())
    }
}
