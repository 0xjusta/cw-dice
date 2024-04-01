use crate::{
    error::ContractError,
    state::{is_owner, OWNER_ADDRESS},
};
use cosmwasm_std::{attr, Addr, DepsMut, MessageInfo, Response};

pub fn handle(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: &Addr,
) -> Result<Response, ContractError> {
    if !is_owner(deps.storage, &info.sender)? {
        return Err(ContractError::Unauthorized {});
    }
    OWNER_ADDRESS.save(deps.storage, new_owner)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "transfer_ownership"),
        attr("new_owner", new_owner.to_string()),
    ]))
}
