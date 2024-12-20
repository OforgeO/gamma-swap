use std::ops::DerefMut;
use std::mem::size_of;

use crate::{
    error::GammaError,
    states::{validate_config_rates, AmmConfig, AMM_CONFIG_SEED},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateAmmConfig<'info> {
    /// Address to be set as protocol owner.
    #[account(
        mut,
        address = crate::admin::id() @ GammaError::InvalidOwner
    )]
    pub owner: Signer<'info>,

    /// Initialize AmmConfig state account to store protocol owner address and fee rates
    #[account(
        init,
        seeds = [
            AMM_CONFIG_SEED.as_bytes(),
            &index.to_be_bytes()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<AmmConfig>()
    )]
    pub amm_config: Account<'info, AmmConfig>,

    pub system_program: Program<'info, System>,
}

pub fn create_amm_config(
    ctx: Context<CreateAmmConfig>,
    index: u16,
    trade_fee_rate: u64,
    protocol_fee_rate: u64,
    fund_fee_rate: u64,
    create_pool_fee: u64,
    max_open_time: u64,
) -> Result<()> {
    let amm_config = ctx.accounts.amm_config.deref_mut();
    amm_config.bump = ctx.bumps.amm_config;
    amm_config.disable_create_pool = false;
    amm_config.index = index;
    amm_config.trade_fee_rate = trade_fee_rate;
    amm_config.protocol_fee_rate = protocol_fee_rate;
    amm_config.fund_fee_rate = fund_fee_rate;
    amm_config.create_pool_fee = create_pool_fee;
    amm_config.protocol_owner = ctx.accounts.owner.key();
    amm_config.fund_owner = ctx.accounts.owner.key();
    amm_config.referral_project = Pubkey::default();
    amm_config.max_open_time = max_open_time;

    validate_config_rates(amm_config)?;

    Ok(())
}
