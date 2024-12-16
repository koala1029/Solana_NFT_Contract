use anchor_lang::prelude::*;
use crate::models::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: AccountInfo<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 32 + 32 + 8 + 8,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        // bumps: &InitializeBumps,
        admin: Pubkey,
        primary_treasury: Pubkey,
        secondary_treasury: Pubkey,
        collection: Pubkey,
    ) -> Result<()> {
        let config = &mut self.config;
        config.admin = admin;
        config.primary_treasury = primary_treasury;
        config.secondary_treasury = secondary_treasury;
        config.collection = collection;
        config.primary_balance = 0;
        config.secondary_balance = 0;
        Ok(())
    }
}
