use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CxwiJ8KMSqrM9JsddTvLT8AECQTXY87poSvd3f4YXhwN");

#[program]
pub mod rust_social {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, display_name: String) -> Result<()>  {
        profile::create_profile(ctx,display_name)
    }

    pub fn create_tweet(ctx:Context<CreateTweet>,body:String) -> Result<()>  {
        tweet::create_tweet(ctx,body)
    }

    pub fn create_like(ctx:Context<CreateLike>) -> Result<()>  {
        tweet::create_like(ctx)
    }


    pub fn create_token_mint_account(ctx:Context<CreateTokenMintAccount>) -> Result<()>  {
        token::create_token_mint_account(ctx)
    }

}
