use cosmwasm_std::{to_json_binary, Binary, Deps, StdResult};

use crate::{msg::GetStatusResponse, state::CONFIG};

pub fn get_status(deps: Deps) -> StdResult<Binary> {
    let config = &CONFIG.load(deps.storage)?;

    to_json_binary(&GetStatusResponse {
        enabled: config.enabled,
        min_bet_amount: config.min_bet_amount,
        max_bet_amount: config.max_bet_amount,
    })
}
