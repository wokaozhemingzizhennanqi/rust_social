use crate::state::tweet::IbuildTweet;
use crate::state::{IbuildLike, IbuildProfile};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};
use crate::{state::token::IbuildToken};

pub fn create_tweet(ctx:Context<CreateTweet>, body:String) -> Result<()> {

    let profile = &mut ctx.accounts.profile;
    profile.tweet_count+=1;
    let tweet = IbuildTweet::new(body,ctx.accounts.authority.key());
    ctx.accounts.tweet.set_inner(tweet.clone());
    Ok(())
}

pub fn create_like(ctx:Context<CreateLike>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    tweet.like_count += 1;

    let like_rel = IbuildLike::new(ctx.accounts.profile.key(), tweet.key());

    let (_, bump) = Pubkey::find_program_address(&[IbuildToken::SEED_PREFIX.as_bytes()], ctx.program_id);
    let seeds = &[IbuildToken::SEED_PREFIX.as_bytes(), &[bump]];
    let signer = &[&seeds[..]];

    mint_to(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.author_token_account.to_account_info(),
            authority: ctx.accounts.mint_account.to_account_info(),

        },
        signer,
    ), 100)?;

    ctx.accounts.like.set_inner(like_rel);
    Ok(())
}


#[derive(Accounts)]
pub struct CreateTweet<'info>{
    #[account(
        init,
        payer = authority,
        space = 8+IbuildTweet::INIT_SPACE,
        seeds = [
        IbuildTweet::SEED_PREFIX.as_bytes(),
        profile.key().as_ref(),
        (profile.tweet_count+1).to_string().as_ref(),
    ],
    bump,
    )]
    pub tweet : Account<'info,IbuildTweet>,
    #[account(
    mut,
    seeds = [
    IbuildProfile::SEED_PREFIX.as_bytes(),
    authority.key().as_ref()
    ],
    bump,
    )]
    pub profile:Account<'info,IbuildProfile>,
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(
        mut,
        seeds=[IbuildToken::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub mint_account: Account<'info,Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = author_wallet,
    )]
    pub author_token_account : Account<'info,TokenAccount>,

    /// CHECK : This is author
    pub author_wallet : AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8+IbuildLike::INIT_SPACE,
        seeds = [
        IbuildLike::SEED_PREFIX.as_bytes(),
        profile.key().as_ref(),
        tweet.key().as_ref(),
    ],
    bump,
    )]
    pub like:Account<'info,IbuildLike>,

    #[account(mut)]
    pub tweet: Account<'info, IbuildTweet>,

    #[account(
        mut,
        seeds = [
        IbuildProfile::SEED_PREFIX.as_bytes(),
        authority.key().as_ref()
        ],
        bump,
    )]
    pub profile: Account<'info, IbuildProfile>,

    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}