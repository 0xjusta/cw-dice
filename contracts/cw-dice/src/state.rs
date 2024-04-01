use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, StdError, StdResult, Storage};
use cw_storage_plus::{Deque, Item, Map};

use crate::{
    models::{BetOrder, Config, Player},
    msg::InstantiateMsg,
    ContractError,
};

pub const REPO_CONTRACT_ADDR: Item<Addr> = Item::new("repo_contract");
pub const OWNER_ADDRESS: Item<Addr> = Item::new("owner_address");
pub const FEE_ADDRESS: Item<Addr> = Item::new("fee_address");

pub const CONFIG: Item<Config> = Item::new("config");
pub const BET_ORDRES: Deque<BetOrder> = Deque::new("bet_orders");
pub const PLAYER_MAP: Map<Addr, Player> = Map::new("player_map");

/// Initialize contract state data.
pub fn initialize(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    msg: &InstantiateMsg,
) -> Result<(), ContractError> {
    REPO_CONTRACT_ADDR.save(deps.storage, &info.sender)?;
    OWNER_ADDRESS.save(deps.storage, &msg.owner)?;
    FEE_ADDRESS.save(deps.storage, &msg.fee_address)?;

    CONFIG.save(
        deps.storage,
        &Config {
            fee_percentage: msg.fee_percentage,
            min_bet_amount: msg.min_bet_amount,
            max_bet_amount: msg.max_bet_amount,
            enabled: true,
        },
    )?;

    Ok(())
}

pub fn is_owner(storage: &dyn Storage, addr: &Addr) -> StdResult<bool> {
    return Ok(OWNER_ADDRESS.load(storage)? == *addr);
}
pub fn is_enabled(storage: &dyn Storage) -> StdResult<bool> {
    return Ok(CONFIG.load(storage)?.enabled);
}
