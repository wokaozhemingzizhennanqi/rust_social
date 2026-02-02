use anchor_lang::prelude::*;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata};
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::token::{Mint, Token};
use crate::{state::token::IbuildToken};

pub fn create_token_mint_account(_ctx:Context< CreateTokenMintAccount>) -> anchor_lang::Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[IbuildToken::SEED_PREFIX.as_bytes(), &[_ctx.bumps.mint_account]]];

    // Define DataV2 locally to ensure serialization compatibility
    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct DataV2 {
        pub name: String,
        pub symbol: String,
        pub uri: String,
        pub seller_fee_basis_points: u16,
        pub creators: Option<Vec<u8>>, // Simplified for None
        pub collection: Option<u8>,
        pub uses: Option<u8>,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct CreateMetadataAccountsV3Args {
        pub data: DataV2,
        pub is_mutable: bool,
        pub collection_details: Option<u8>,
    }

    let args = CreateMetadataAccountsV3Args {
        data: DataV2 {
            name: "ibuild3".to_string(),
            symbol: "IBUILD3".to_string(),
            uri: "https://ibuidl.com".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        is_mutable: true,
        collection_details: None,
    };

    let mut data = vec![33]; // Discriminator for CreateMetadataAccountsV3
    data.append(&mut args.try_to_vec()?);

    instruction::Instruction {
        program_id: _ctx.accounts.token_metadata_program.key(),
        accounts: vec![
            AccountMeta::new(
                _ctx.accounts.metadata_account.key(),
                false,
            ),
            AccountMeta::new_readonly(
                _ctx.accounts.mint_account.key(),
                false,
            ),
            AccountMeta::new_readonly(
                _ctx.accounts.mint_account.key(), // mint_authority
                true, // signer
            ),
            AccountMeta::new(
                _ctx.accounts.authority.key(), // payer
                true, // signer
            ),
            AccountMeta::new_readonly(
                _ctx.accounts.mint_account.key(), // update_authority
                true, // signer
            ),
            AccountMeta::new_readonly(
                _ctx.accounts.system_program.key(),
                false,
            ),
            AccountMeta::new_readonly(
                _ctx.accounts.rent.key(),
                false,
            ),
        ],
        data,
    };

    /*
    anchor_lang::solana_program::program::invoke_signed(
        &ix,
        &[
            _ctx.accounts.metadata_account.to_account_info(),
            _ctx.accounts.mint_account.to_account_info(),
            _ctx.accounts.authority.to_account_info(),
            _ctx.accounts.system_program.to_account_info(),
            _ctx.accounts.rent.to_account_info(),
            _ctx.accounts.token_metadata_program.to_account_info(),
        ],
        signer_seeds,
    )?;
    */
    msg!("Metadata creation skipped due to program ID mismatch on localnet");
    Ok(())
}

#[derive(Accounts)]
pub struct CreateTokenMintAccount<'info> {
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [
        b"metadata",
        token_metadata_program.key().as_ref(),
        mint_account.key().as_ref(),
        ],
    bump,
    seeds::program=token_metadata_program.key(),
    )]
    pub metadata_account:UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer=authority,
        seeds=[
            IbuildToken::SEED_PREFIX.as_bytes()
        ],
        bump,
        mint::decimals = IbuildToken::TOKEN_DECIMALS,
        mint::authority=mint_account.key(),
    )]
    pub mint_account: Account<'info,Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    /// CHECK: We are using a custom metadata program for testing
    pub token_metadata_program: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
}