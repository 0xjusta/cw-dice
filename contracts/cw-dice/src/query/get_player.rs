use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, StdResult};

use crate::{models::Player, msg::GetPlayerResponse, state::PLAYER_MAP};

pub fn get_player(deps: Deps, wallet: Addr) -> StdResult<Binary> {
    let player = &PLAYER_MAP.load(deps.storage, wallet)?;
    to_json_binary(&GetPlayerResponse {
        player: Player {
            times: player.times,
            last_timestamp: player.last_timestamp,
        },
    })
}
