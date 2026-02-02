use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};
use crate::{state::token::IbuildToken};
use crate::state::IbuildStake;

pub fn stake(ctx:Context<NftStake>) -> Result<()>{
    // 记录质押关系
    let stake_info = IbuildStake::new(ctx.accounts.authority.key(),ctx.accounts.nft_mint_account.key());
    ctx.accounts.stake_info.set_inner(stake_info);
    // 转移NFT
    transfer(CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer{
            from:ctx.accounts.nft_associated_token_account.to_account_info(),
            to:ctx.accounts.program_receipt_nft_ata.to_account_info(),
            authority:ctx.accounts.authority.to_account_info(),
        }
    ),1)?;

    // mint 流动性代币
    let singer_seeds: &[&[&[u8]]] = &[&[IbuildToken::SEED_PREFIX.as_bytes(), &[ctx.bumps.token_mint_account]]];
    mint_to(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo{
            mint:ctx.accounts.token_mint_account.to_account_info(),
            to:ctx.accounts.associated_token_account.to_account_info(),
            authority:ctx.accounts.token_mint_account.to_account_info(),
        },
        singer_seeds
    ),10000)?;
    Ok(())
}


#[derive(Accounts)]
pub struct NftStake<'info> {
    // 要接收NFT的账户 ////
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + IbuildStake::INIT_SPACE,
        seeds=[
            IbuildStake::SEED_PREFIX.as_bytes(),
            token_mint_account.key().as_ref(),
        ],
        bump,
    )]
    pub stake_info:Box<Account<'info,IbuildStake>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = stake_info,
    )]
    pub program_receipt_nft_ata:Box<Account<'info,TokenAccount>>,
    ///// 流动性token //////
    // 检查token是不是合约的派生pda
    #[account(
        mut,
        seeds=[
            IbuildToken::SEED_PREFIX.as_bytes()
        ],
        bump,
    )]
    pub token_mint_account: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint_account,
        associated_token::authority = authority,
    )]
    pub associated_token_account: Box<Account<'info,TokenAccount>>,

    //// 质押NFT /////
    #[account(
        mut,
    )]
    pub nft_mint_account : Box<Account<'info,Mint>>,

    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account : Box<Account<'info,TokenAccount>>,

    #[account(mut)]
    authority: Signer<'info>,

    pub associated_token_program: Program<'info,AssociatedToken>,

    pub token_program: Program<'info,Token>,

    pub system_program: Program<'info, System>,
}