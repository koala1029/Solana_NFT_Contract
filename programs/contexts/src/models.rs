use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub primary_treasury: Pubkey,
    pub secondary_treasury: Pubkey,
    pub collection: Pubkey,
    pub primary_balance: u64,
    pub secondary_balance: u64,
}

#[account]
pub struct NftInfo {
    pub id: u64,
    pub address: Pubkey,
    pub collection: Pubkey,
}
