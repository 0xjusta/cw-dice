#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Coin, Decimal, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
    SubMsg, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    create_bet, deposit_house, transfer_ownership, update_setting, withdraw_house,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use crate::state::{self};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-dice";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    state::initialize(deps, &_env, &info, &msg)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership { new_owner } => {
            transfer_ownership::handle(deps, info, &new_owner)
        }
        ExecuteMsg::UpdateSetting {
            enabled,
            min_bet_amount,
            max_bet_amount,
        } => update_setting::handle(deps, info, enabled, min_bet_amount, max_bet_amount),
        ExecuteMsg::CreateBet { side, amount } => create_bet::handle(deps, env, info, side, amount),
        ExecuteMsg::DepositHouse {} => deposit_house::handle(deps, info),
        ExecuteMsg::WithdrawHouse { amount } => withdraw_house::handle(deps, info, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStatus {} => query::get_status(deps),
        QueryMsg::GetHistory { cursor, limit } => query::get_history(deps, cursor, limit),
        QueryMsg::GetPlayer { address } => query::get_player(deps, address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do any desired state migrations...

    Ok(Response::default())
}
