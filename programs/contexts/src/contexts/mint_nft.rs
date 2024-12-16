use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_spl::metadata::mpl_token_metadata::{
    instructions::{
        CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
        CreateMasterEditionV3InstructionArgs, CreateMetadataAccountV3Cpi,
        CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs,
    },
    types::{Collection, Creator, DataV2},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::errors::ErrorCode;
use crate::events::*;
use crate::models::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This account will be initialized by the metaplex program
    pub master_edition: UncheckedAccount<'info>,
    #[account(
        seeds = [b"authority"],
        bump,
    )]
    /// CHECK: This is account is not initialized and is being used for signing purposes only
    pub mint_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + 8 + 32 + 32,
        seeds = ["nftinfo".as_bytes(), id.to_le_bytes().as_ref()],
        bump,
    )]
    pub nft_info: Account<'info, NftInfo>,
    #[account(
        mut,
        seeds = [b"vault"],
        bump,
    )]
    pub vault: AccountInfo<'info>,
    #[account(
        mut,
        seeds = ["config".as_bytes()], 
        bump,
    )]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub collection_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(&mut self, bumps: &MintNFTBumps, id: u64) -> Result<()> {
        let metadata = &self.metadata.to_account_info();
        let master_edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let authority = &self.mint_authority.to_account_info();
        let payer = &self.owner.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();
        let spl_metadata_program = &self.token_metadata_program.to_account_info();

        require!(
            self.config.collection.key() == self.collection_mint.key(),
            ErrorCode::UnauthorizedCollection
        );
        let ix = system_instruction::transfer(&self.owner.key(), &self.vault.key(), 500000000);

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[self.owner.to_account_info(), self.vault.to_account_info()],
        )?;

        let config = &mut self.config;
        config.primary_balance += 475000000;
        config.secondary_balance += 25000000;

        let nft_info = &mut self.nft_info;
        nft_info.id = id;
        nft_info.address = self.mint.key();
        nft_info.collection = self.collection_mint.key();

        let seeds = &[&b"authority"[..], &[bumps.mint_authority]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        mint_to(cpi_ctx, 1)?;
        msg!("Collection NFT minted!");

        let creator = vec![Creator {
            address: self.mint_authority.key(),
            verified: true,
            share: 100,
        }];

        let name = format!("ASQUIDS #{}", id);
        let symbol = "ASQUIDS".to_string();
        let base_uri = "https://ipfs.io/ipfs/QmV7gwejLDJyzi7Qm4XudMCH25Xy8CRS3btWNu7s3BjdHp/";
        let uri = format!("{}{}.json", base_uri, id);

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            spl_metadata_program,
            CreateMetadataAccountV3CpiAccounts {
                metadata,
                mint,
                mint_authority: authority,
                payer,
                update_authority: (authority, true),
                system_program,
                rent: None,
            },
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name,
                    symbol,
                    uri,
                    seller_fee_basis_points: 1000,
                    creators: Some(creator),
                    collection: Some(Collection {
                        verified: false,
                        key: self.collection_mint.key(),
                    }),
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        );
        metadata_account.invoke_signed(signer_seeds)?;

        let master_edition_account = CreateMasterEditionV3Cpi::new(
            spl_metadata_program,
            CreateMasterEditionV3CpiAccounts {
                edition: master_edition,
                update_authority: authority,
                mint_authority: authority,
                mint,
                payer,
                metadata,
                token_program: spl_token_program,
                system_program,
                rent: None,
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(0),
            },
        );
        master_edition_account.invoke_signed(signer_seeds)?;

        emit!(NFTMinted {
            mint_address: mint.key(),
            owner: payer.key(),
        });

        Ok(())
    }
}
