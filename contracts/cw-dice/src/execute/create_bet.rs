use crate::{
    error::ContractError,
    helpers::{calc_platform_fee, get_random, INJ_DENOM},
    models::{BetOrder, Player},
    state::{is_enabled, BET_ORDRES, CONFIG, FEE_ADDRESS, PLAYER_MAP, REPO_CONTRACT_ADDR},
};
use cosmwasm_std::{
    attr, coin, BalanceResponse, BankMsg, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128,
};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    side: u8,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Game should be enabled
    if !is_enabled(deps.storage)? {
        return Err(ContractError::Paused {});
    }

    let player = info.sender;

    let config = CONFIG.load(deps.storage)?;
    let fee_wallet = FEE_ADDRESS.load(deps.storage)?;
    let contract_address = REPO_CONTRACT_ADDR.load(deps.storage)?;
    // let bet_orders = BET_ORDRES.load(deps.storage)?;

    let mut msgs: Vec<CosmosMsg> = vec![];

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
    if amount < config.min_bet_amount || amount > config.max_bet_amount {
        return Err(ContractError::InvalidAmount {});
    }

    let fee_amount = calc_platform_fee(amount, config.fee_percentage)?;

    // Validate sent amount is corrent
    if funds.amount < (amount.checked_add(fee_amount).unwrap()) {
        return Err(ContractError::AmountNotEnough {});
    }

    // Transfer fee to fee wallet
    msgs.push(
        BankMsg::Send {
            to_address: fee_wallet.to_string(),
            amount: vec![coin(fee_amount.u128(), INJ_DENOM)],
        }
        .into(),
    );

    // Generate random seed
    let rand: u64 = get_random(&env, &player.to_string());
    let dice = (rand % 6 + 1) as u8;
    let won = (dice % 2) == side;

    // If win, then transfer 2x funds from house to player
    if won {
        msgs.push(
            BankMsg::Send {
                to_address: player.to_string(),
                amount: vec![coin(amount.u128() * 2, INJ_DENOM)],
            }
            .into(),
        );
    }

    // Add play history
    let now = env.block.time.seconds();
    BET_ORDRES.push_back(
        deps.storage,
        &BetOrder {
            address: player.to_string(),
            side,
            amount,
            dice,
            won,
            ts: now,
        },
    )?;

    // Update play history
    PLAYER_MAP.update(deps.storage, player.clone(), |data| -> StdResult<Player> {
        match data {
            Some(data) => Ok(Player {
                times: data.times + 1,
                last_timestamp: now,
            }),
            None => Ok(Player {
                times: 1,
                last_timestamp: now,
            }),
        }
    })?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "create_bet"),
            attr("sender", player.to_string()),
            attr("side", side.to_string()),
            attr("amount", amount.to_string()),
            attr("result", dice.to_string()),
        ])
        .add_messages(msgs))
}
