use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub enum BetStatus {
    Pending,
    Complete,
    Canceled,
}

#[cw_serde]
pub struct Config {
    pub fee_percentage: u32,
    pub min_bet_amount: Uint128,
    pub max_bet_amount: Uint128,
    pub enabled: bool,
}

#[cw_serde]
pub struct BetOrder {
    pub address: String,
    pub amount: Uint128,
    pub side: u8,
    pub dice: u8,
    pub won: bool,
    pub ts: u64,
}

#[cw_serde]
pub struct Player {
    pub times: u64,
    pub last_timestamp: u64,
}
