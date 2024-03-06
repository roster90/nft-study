use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("6QPrpaMJVtvMqmd62GpuEDMJKj2DmxzeXuRTnDqww2Xv");

#[program]
pub mod nft_study {
    use anchor_spl::{metadata::{create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3, CreateMetadataAccountsV3}, token::{mint_to, MintTo}};
    use mpl_token_metadata::types::{Collection, CollectionDetails, Creator, DataV2};

    use super::*;
    pub const SEEDS_MINT_NFT: &[u8] = b"MINT_NFT";
    pub const SEEDS_MINT_COLLECTION: &[u8] = b"MINT_COLLECTION";


    pub fn create_nft(ctx: Context<CreateNFT>, name: String, symbol: String, uri: String, id: u64) -> Result<()> {
        let id_bytes = id.to_le_bytes();
        let seeds = &[
            "mint".as_bytes(),
            id_bytes.as_ref(),
            &[ctx.bumps.mint],
        ];
        msg!("Run mint_to");
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1, // 1 token
        )?;


        let creators: Vec<Creator> = vec![Creator {
            address: ctx.accounts.authority.key(),
            verified: false,
            share: 100,
        }];

        let collections: Collection = Collection {
            verified: false,
            key: ctx.accounts.nft_metadata.key(),
        };
        //create metadata account
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: ctx.accounts.authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.authority.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 550, //tien share ban quyen cho nguoi tao (550 = 5,5%)
                creators: Some(creators), //nguoi tao limit 5
                collection: Some(collections), //thuoc collection nao
                uses: None,
            },
            true,
            true,
            None,
        )?;
        msg!("Run create master edition v3");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    payer: ctx.accounts.authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.authority.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            Some(1),
        )?;

        msg!("Minted NFT successfully");

        Ok(())
    }


    pub fn create_collection_nft( ctx: Context<CreateCollectionNFT>, name: String, symbol: String, uri: String) -> Result<()>{

        let name_byte = name.clone();
        let seeds = &[
            "MINT_COLLECTION".as_bytes(),
            name_byte.as_bytes(),
            &[ctx.bumps.mint],
        ];

        msg!("Run mint_to");
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.authority.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1, // 1 token
        )?;

        let collection_details = CollectionDetails::V1 { size: 100};
       
       
        let creators: Vec<Creator> = vec![Creator {
            address: ctx.accounts.authority.key(),
            verified: true,
            share: 100,
        }];


        //create metadata account
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    payer: ctx.accounts.authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.authority.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0, //tien share ban quyen cho nguoi tao (550 = 5,5%)
                creators: Some(creators), //nguoi tao limit 5
                collection: None,
                uses: None,
                
            },
            true,
            true,
            None,
        )?;

       
        msg!("Run create master edition v3");

        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    edition: ctx.accounts.master_edition_account.to_account_info(),
                    payer: ctx.accounts.authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint_authority: ctx.accounts.authority.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &[&seeds[..]],
            ),
            Some(1),
        )?;

        msg!("Minted NFT successfully");

        Ok(())
    }


    pub fn nft_transfer( ctx: Context<TransferNFTToken>)->Result<()>{
        Ok(())
    
    }

}

#[derive(Accounts)]
#[instruction( name: String, symbol: String, uri: String, id: u64)]
pub struct CreateNFT<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key(),
        seeds = [b"MINT_NFT", id.to_le_bytes().as_ref()], 
        bump,
    )]
    mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>, // new
    /// CHECK: New Metaplex Account being created
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>, // new
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>, // new
    pub associated_token_program: Program<'info, AssociatedToken>, // new
    pub system_program: Program<'info, System>, // bew
    pub rent: Sysvar<'info, Rent> ,// new
      /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
}
#[derive(Accounts)]
#[instruction( name: String, symbol: String, uri: String)]
pub struct CreateCollectionNFT<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority.key(),
        mint::freeze_authority = authority.key(),
        seeds = [b"MINT_COLLECTION", name.as_bytes()], 
        bump,
    )]
    mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>, // new
    /// CHECK: New Metaplex Account being created
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(),
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK:
    pub master_edition_account: UncheckedAccount<'info>, // new
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>, // new
    pub associated_token_program: Program<'info, AssociatedToken>, // new
    pub system_program: Program<'info, System>, // bew
    pub rent: Sysvar<'info, Rent> ,// new
      /// CHECK: account constraint checked in account trait
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,
}
#[derive(Accounts)]
pub struct TransferNFTToken<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub to: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    ///CHECK: only to create a new  ata account
    pub to_address : UncheckedAccount<'info>,
    #[account(mut,signer)]
    pub authority: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]

pub struct InitNFTParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}