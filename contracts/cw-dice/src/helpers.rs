use cosmwasm_std::{Env, Uint128};
use sha2::Digest;

use crate::error::ContractError;

pub const INJ_DENOM: &str = "inj";

pub fn calc_platform_fee(amount: Uint128, fee_percentage: u32) -> Result<Uint128, ContractError> {
    let f = amount
        .checked_mul(Uint128::from(fee_percentage))?
        .checked_div(Uint128::from(100_00 as u32))?;
    Ok(f)
}

pub fn get_random(env: &Env, sender: &String) -> u64 {
    let tx_index = if let Some(tx) = &env.transaction {
        tx.index
    } else {
        0
    };

    let rng_seeds = format!(
        "{}-i-{}-n-{}-j-{}",
        tx_index,
        env.block.height,
        env.block.time.nanos(),
        sender
    );

    let mut hasher = sha2::Sha256::new();
    hasher.update(rng_seeds.as_str());
    hasher.finalize().iter().fold(0, |acc, x| acc + *x as u64)
}
