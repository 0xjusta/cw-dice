use cosmwasm_std::{to_json_binary, Addr, Binary, Deps, StdResult};

use crate::{
    models::BetOrder,
    msg::GetHistoryResponse,
    state::{BET_ORDRES, CONFIG},
};

pub fn get_history(deps: Deps, _cursor: u32, limit: u32) -> StdResult<Binary> {
    let limit = if limit > 100 { 100 } else { limit };

    let mut orders: Vec<BetOrder> = vec![];

    let mut iter = BET_ORDRES.iter(deps.storage).unwrap().rev();

    for _ in 0..limit {
        let order = iter.next();
        if order.is_none() {
            break;
        }

        orders.push(order.unwrap().unwrap());
    }

    to_json_binary(&GetHistoryResponse { orders })
}
