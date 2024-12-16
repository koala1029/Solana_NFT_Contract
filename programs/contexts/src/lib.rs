use anchor_lang::prelude::*;

declare_id!("Aq3BJbaCtNTinYS2YWtzWyADX9oY5SXRP6MFu8JzRZy3");

pub mod contexts;
pub mod errors;
pub mod events;
pub mod models;

pub use contexts::*;
pub use errors::*;
pub use events::*;
pub use models::*;

#[program]
pub mod mint_nft {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        admin: Pubkey,
        primary_treasury: Pubkey,
        secondary_treasury: Pubkey,
        collection: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .initialize(admin, primary_treasury, secondary_treasury, collection)
        // .initialize(&ctx.bumps, admin, primary_treasury, secondary_treasury)
    }

    pub fn create_collection(ctx: Context<CreateCollection>) -> Result<()> {
        ctx.accounts.create_collection(&ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNFT>, id: u64) -> Result<()> {
        ctx.accounts.mint_nft(&ctx.bumps, id)
    }

    pub fn verify_collection(ctx: Context<VerifyCollectionMint>) -> Result<()> {
        ctx.accounts.verify_collection(&ctx.bumps)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(&ctx.bumps, amount)
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        admin: Pubkey,
        primary_treasury: Pubkey,
        secondary_treasury: Pubkey,
        collection: Pubkey,
    ) -> Result<()> {
        ctx.accounts.update_config(
            &ctx.bumps,
            admin,
            primary_treasury,
            secondary_treasury,
            collection,
        )
    }
}
