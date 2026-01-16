use anchor_lang::prelude::*;
use crate::state::{IbuildLike, IbuildProfile};
use crate::state::tweet::IbuildTweet;

pub fn create_tweet(ctx:Context<CreateTweet>, body:String) -> Result<()> {

    let profile = &mut ctx.accounts.profile;
    profile.tweet_count+=1;
    let tweet = IbuildTweet::new(body);
    ctx.accounts.tweet.set_inner(tweet.clone());
    Ok(())
}

pub fn create_like(ctx:Context<CreateLike>) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    tweet.like_count+=1;

    let like_rel = IbuildLike::new(ctx.accounts.profile.key(),tweet.key());

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
        init,
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

}