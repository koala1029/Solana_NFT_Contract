use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::models::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> UpdateConfig<'info> {
    pub fn update_config(
        &mut self,
        _bumps: &UpdateConfigBumps,
        admin: Pubkey,
        primary_treasury: Pubkey,
        secondary_treasury: Pubkey,
        collection: Pubkey,
    ) -> Result<()> {
        let config = &mut self.config;
        let user = &mut self.user;
        require!(config.admin == user.key(), ErrorCode::UnauthorizedCaller);
        config.admin = admin;
        config.primary_treasury = primary_treasury;
        config.secondary_treasury = secondary_treasury;
        config.collection = collection;

        Ok(())
    }
}
