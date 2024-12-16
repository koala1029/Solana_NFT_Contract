use anchor_lang::prelude::*;

#[event]
pub struct NFTMinted {
    pub mint_address: Pubkey,
    pub owner: Pubkey,
}
