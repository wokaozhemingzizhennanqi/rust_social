use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct IbuildToken {

}

impl IbuildToken {
    pub const SEED_PREFIX: &'static str = "mint_v21";

    pub const TOKEN_DECIMALS: u8 = 2;
}