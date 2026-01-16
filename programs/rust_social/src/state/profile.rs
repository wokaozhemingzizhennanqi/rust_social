use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct IbuildProfile {
    #[max_len(20)]
    pub display_name: String,

    pub tweet_count: u32,
}

impl IbuildProfile {
    pub const SEED_PREFIX:&'static str = "profile";
}