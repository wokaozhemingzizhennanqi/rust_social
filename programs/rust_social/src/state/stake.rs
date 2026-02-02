use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct IbuildStake {
    pub staker: Pubkey,
    pub nft_mint_account: Pubkey,
    pub staked_at : u64, //质押的时间
}

impl IbuildStake {
    pub const SEED_PREFIX: &'static str = "stake_v1";

    pub fn new(staker:Pubkey, nft_mint_account: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        let staked_at = clock.epoch;

        Self{
            staker,
            nft_mint_account,
            staked_at,
        }
    }
}