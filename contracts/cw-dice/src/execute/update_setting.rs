use crate::{
    error::ContractError,
    state::{is_owner, CONFIG},
};
use cosmwasm_std::{attr, DepsMut, MessageInfo, Response, StdResult, Uint128};

pub fn handle(
    deps: DepsMut,
    info: MessageInfo,
    enabled: Option<bool>,
    min_bet_amount: Option<Uint128>,
    max_bet_amount: Option<Uint128>,
) -> Result<Response, ContractError> {
    if !is_owner(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    if enabled.is_some() {
        CONFIG.update(deps.storage, |mut c| -> StdResult<_> {
            c.enabled = enabled.unwrap();
            Ok(c)
        })?;
    }

    if min_bet_amount.is_some() {
        CONFIG.update(deps.storage, |mut c| -> StdResult<_> {
            c.min_bet_amount = min_bet_amount.unwrap();
            Ok(c)
        })?;
    }

    if max_bet_amount.is_some() {
        CONFIG.update(deps.storage, |mut c| -> StdResult<_> {
            c.max_bet_amount = max_bet_amount.unwrap();
            Ok(c)
        })?;
    }

    Ok(Response::new().add_attributes(vec![attr("action", "update_setting")]))
}
