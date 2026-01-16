use anchor_lang::prelude::*;
use crate::state::IbuildProfile;


#[account]
#[derive(InitSpace)]
pub struct IbuildLike{
    pub profile_pubkey: Pubkey,
    pub tweet_pubkey: Pubkey,

}

impl IbuildLike{

    pub const SEED_PREFIX:&'static str = "like";

    pub fn new(profile_pubkey:Pubkey,tweet_pubkey:Pubkey) -> Self{
        Self{profile_pubkey, tweet_pubkey}
    }

}