use crate::{error::ContractError, helpers::INJ_DENOM, state::is_owner};
use cosmwasm_std::{attr, coin, BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, Uint128};

pub fn handle(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Owner can only withdraw
    if !is_owner(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    // Validate amount
    if amount.is_zero() {
        return Err(ContractError::InvalidAmount {});
    }

    let mut msgs: Vec<CosmosMsg> = vec![];

    // Transfer funds to owner wallet
    msgs.push(
        BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(amount.u128(), INJ_DENOM)],
        }
        .into(),
    );

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "withdraw_house"),
            attr("sender", info.sender.to_string()),
            attr("amount", amount.to_string()),
        ])
        .add_messages(msgs))
}
