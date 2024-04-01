use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

use crate::models::{BetOrder, Player};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub fee_address: Addr,
    pub fee_percentage: u32,
    pub min_bet_amount: Uint128,
    pub max_bet_amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnership {
        new_owner: Addr,
    },
    UpdateSetting {
        enabled: Option<bool>,
        min_bet_amount: Option<Uint128>,
        max_bet_amount: Option<Uint128>,
    },
    CreateBet {
        side: u8,
        amount: Uint128,
    },
    DepositHouse {},
    WithdrawHouse {
        amount: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetStatusResponse)]
    GetStatus {},

    #[returns(GetHistoryResponse)]
    GetHistory { limit: u32, cursor: u32 },

    #[returns(GetPlayerResponse)]
    GetPlayer { address: Addr },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetStatusResponse {
    pub enabled: bool,
    pub min_bet_amount: Uint128,
    pub max_bet_amount: Uint128,
}

#[cw_serde]
pub struct GetHistoryResponse {
    pub orders: Vec<BetOrder>,
}

#[cw_serde]
pub struct GetPlayerResponse {
    pub player: Player,
}
