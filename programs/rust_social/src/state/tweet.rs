use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct  IbuildTweet{

    pub like_count: u64,
    pub author:Pubkey,
    #[max_len(50)]
    pub body:String
}


impl IbuildTweet {
    pub const SEED_PREFIX:&'static str = "tweet";


    pub fn new(body:String,author:Pubkey) -> Self {
        Self{like_count:0,body,author}
    }
}