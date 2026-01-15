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


}
