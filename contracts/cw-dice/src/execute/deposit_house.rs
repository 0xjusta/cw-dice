use crate::{error::ContractError, helpers::INJ_DENOM, state::is_owner};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

pub fn handle(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    // Owner can only deposit
    if !is_owner(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    // Verify we only have one coin sent
    if info.funds.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    }

    let funds = info.funds[0].clone();
    // Validate funds is INJ
    if !funds.denom.eq(INJ_DENOM) {
        return Err(ContractError::InvalidFunds {});
    }

    // Validate amount
    if funds.amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    Ok(Response::new().add_attributes(vec![
        attr("action", "deposit_house"),
        attr("sender", info.sender.to_string()),
        attr("amount", funds.amount.to_string()),
    ]))
}
