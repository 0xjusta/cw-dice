pub mod utils;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::msg::{
        ExecuteMsg, GetHistoryResponse, GetPlayerResponse, GetStatusResponse, InstantiateMsg,
        QueryMsg,
    };

    use super::*;
    use cosmwasm_std::{coin, coins, testing::mock_info, Addr, BlockInfo, Timestamp, Uint128};
    use cw_multi_test::Executor;
    use tests::utils::{
        get_balance, setup_base_contract, FEE_TREASURY_ADDR, NATIVE_DENOM, OWNER_ADDR,
        PLATFORM_FEE_RATE, PLAYER1_ADDR, PLAYER2_ADDR,
    };

    #[test]
    fn proper_initialization() {
        let (app, contract_addr) = setup_base_contract();
        let res: GetStatusResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::GetStatus {})
            .unwrap();

        assert_eq!(true, res.enabled);
    }

    #[test]
    fn test_transfer_ownership() {
        let new_owner = mock_info("new_owner", &coins(1000, "earth"));

        let (mut app, contract_addr) = setup_base_contract();

        let res = app.execute_contract(
            Addr::unchecked(OWNER_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::TransferOwnership {
                new_owner: new_owner.sender.clone(),
            },
            &vec![],
        );
        assert_eq!(true, res.is_ok());

        // revert to initial owner
        let res = app.execute_contract(
            new_owner.sender,
            contract_addr.clone(),
            &ExecuteMsg::TransferOwnership {
                new_owner: Addr::unchecked(OWNER_ADDR),
            },
            &vec![],
        );
        assert_eq!(true, res.is_ok());
    }

    #[test]
    fn test_update_setting() {
        let (mut app, contract_addr) = setup_base_contract();

        let _max_bet_amount = Uint128::from(500 as u128);
        let res = app.execute_contract(
            Addr::unchecked(OWNER_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::UpdateSetting {
                enabled: Some(false),
                min_bet_amount: None,
                max_bet_amount: Some(_max_bet_amount),
            },
            &vec![],
        );
        assert_eq!(true, res.is_ok());

        let res: GetStatusResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::GetStatus {})
            .unwrap();
        assert_eq!(res.max_bet_amount, _max_bet_amount);
    }

    #[test]
    fn test_create_bet() {
        let (mut app, contract_addr) = setup_base_contract();

        // Query fee wallet balance
        let fee_balance_before = get_balance(&app, FEE_TREASURY_ADDR.to_string());
        let player_balance_before = get_balance(&app, PLAYER1_ADDR.to_string());

        let side = 1;
        let bet_amount: u128 = 1000;
        let fee_amount: u128 = bet_amount * PLATFORM_FEE_RATE / 10000;

        let res = app.execute_contract(
            Addr::unchecked(PLAYER1_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::CreateBet {
                side: 1,
                amount: Uint128::from(bet_amount),
            },
            &[coin(bet_amount + fee_amount, NATIVE_DENOM)],
        );

        assert_eq!(true, res.is_ok());

        let ret = res.unwrap();
        let result = ret
            .events
            .iter()
            .flat_map(|t| t.attributes.clone())
            .find(|t| t.key.eq("result"))
            .unwrap()
            .value
            .parse::<u8>()
            .unwrap();
        let is_win = result % 2 == side;
        println!("{}, {}", result, is_win);

        // Check fee balance
        let fee_balance_after = get_balance(&app, FEE_TREASURY_ADDR.to_string());
        assert_eq!((fee_balance_after - fee_balance_before), fee_amount);

        let player_balance_after = get_balance(&app, PLAYER1_ADDR.to_string());
        if is_win {
            // win case, should be player balance increased
            assert_eq!(
                (player_balance_after - player_balance_before),
                bet_amount - fee_amount
            );
        }
    }

    #[test]
    fn test_create_bet_2() {
        let (mut app, contract_addr) = setup_base_contract();

        app.set_block(BlockInfo {
            height: 100,
            time: Timestamp::from_nanos(1000),
            chain_id: "inj".to_string(),
        });
        // Query fee wallet balance
        let fee_balance_before = get_balance(&app, FEE_TREASURY_ADDR.to_string());
        let house_balance_before = get_balance(&app, contract_addr.to_string());

        let side = 1;
        let bet_amount: u128 = 1_000_000_000_000_000_000;
        let fee_amount: u128 = bet_amount * PLATFORM_FEE_RATE / 10000;

        let res = app.execute_contract(
            Addr::unchecked(PLAYER2_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::CreateBet {
                side,
                amount: Uint128::from(bet_amount),
            },
            &[coin(bet_amount + fee_amount + 1_0000, NATIVE_DENOM)],
        );
        println!("{:?}", res);
        assert_eq!(true, res.is_ok());

        let ret = res.unwrap();
        let result = ret
            .events
            .iter()
            .flat_map(|t| t.attributes.clone())
            .find(|t| t.key.eq("result"))
            .unwrap()
            .value
            .parse::<u8>()
            .unwrap();
        let is_win = result % 2 == side;
        println!("{}, {}", result, is_win);

        // Check fee balance
        let fee_balance_after = get_balance(&app, FEE_TREASURY_ADDR.to_string());
        assert_eq!((fee_balance_after - fee_balance_before), fee_amount,);

        let house_balance_after = get_balance(&app, contract_addr.to_string());

        if !is_win {
            // lost case, should be house balance increased
            assert_eq!((house_balance_after - house_balance_before), bet_amount);
        }
    }

    #[test]
    fn test_deposit_house() {
        let (mut app, contract_addr) = setup_base_contract();

        // Query wallet balance
        let house_balance_before = get_balance(&app, contract_addr.to_string());

        let amount: u128 = 10000;

        // Only owner can deposit
        let res = app.execute_contract(
            Addr::unchecked(PLAYER2_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::DepositHouse {},
            &[coin(amount, NATIVE_DENOM)],
        );
        assert_eq!(true, res.is_err());

        let res = app.execute_contract(
            Addr::unchecked(OWNER_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::DepositHouse {},
            &[coin(amount, NATIVE_DENOM)],
        );
        assert_eq!(true, res.is_ok());

        let house_balance_after = get_balance(&app, contract_addr.to_string());
        // lost case, should be house balance increased
        assert_eq!((house_balance_after - house_balance_before), amount);
    }

    #[test]
    fn test_withdraw_house() {
        let (mut app, contract_addr) = setup_base_contract();

        // Query wallet balance
        let owner_balance_before = get_balance(&app, OWNER_ADDR.to_string());

        let amount: u128 = 10000;

        // Only owner can withdraw
        let res = app.execute_contract(
            Addr::unchecked(PLAYER2_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::WithdrawHouse {
                amount: Uint128::from(amount),
            },
            &[],
        );
        assert_eq!(true, res.is_err());

        let res = app.execute_contract(
            Addr::unchecked(OWNER_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::WithdrawHouse {
                amount: Uint128::from(amount),
            },
            &[],
        );
        assert_eq!(true, res.is_ok());

        let owner_balance_after = get_balance(&app, OWNER_ADDR.to_string());
        // lost case, should be owner balance increased
        assert_eq!((owner_balance_after - owner_balance_before), amount);
    }

    #[test]
    fn test_query_bet_history() {
        let (mut app, contract_addr) = setup_base_contract();

        // Query fee wallet balance
        let side = 1;
        let bet_amount: u128 = 1000;
        let fee_amount: u128 = bet_amount * PLATFORM_FEE_RATE / 10000;

        let res = app.execute_contract(
            Addr::unchecked(PLAYER1_ADDR),
            contract_addr.clone(),
            &ExecuteMsg::CreateBet {
                side,
                amount: Uint128::from(bet_amount),
            },
            &[coin(bet_amount + fee_amount, NATIVE_DENOM)],
        );
        assert_eq!(true, res.is_ok());

        let res: GetPlayerResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::GetPlayer {
                    address: Addr::unchecked(PLAYER1_ADDR),
                },
            )
            .unwrap();
        println!("{:?}", res);

        let res: GetHistoryResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(),
                &QueryMsg::GetHistory {
                    limit: 10,
                    cursor: 0,
                },
            )
            .unwrap();
        println!("{:?}", res);
    }
}
