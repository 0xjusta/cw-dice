use cosmwasm_std::{coin, testing::MockApi, Addr, Empty, MemoryStorage, Uint128};
use cw_multi_test::{
    App, BankKeeper, BasicAppBuilder, Contract, ContractWrapper, Executor, FailingModule,
    WasmKeeper,
};

use crate::msg::InstantiateMsg;

pub const OWNER_ADDR: &str = "owner";
pub const FEE_TREASURY_ADDR: &str = "fee";
pub const PLAYER1_ADDR: &str = "player1";
pub const PLAYER2_ADDR: &str = "player2";
pub const PLAYER3_ADDR: &str = "player3";

pub const PLATFORM_FEE_RATE: u128 = 400;
pub const MIN_BET_AMOUNT: u64 = 1_000;
pub const MAX_BET_AMOUNT: u64 = 1_000_000_000_000_000_000;

pub const NATIVE_DENOM: &str = "inj";

pub type BaseApp = App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    FailingModule<Empty, Empty, Empty>,
    WasmKeeper<Empty, Empty>,
>;

fn contract_token() -> Box<dyn Contract<Empty, Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );

    Box::new(contract)
}

pub fn get_balance(app: &BaseApp, addr: String) -> u128 {
    app.wrap()
        .query_balance(addr, NATIVE_DENOM)
        .unwrap()
        .amount
        .u128()
}

pub fn add_balance(app: &mut BaseApp, addr: Addr, amount: u128) {
    app.init_modules(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &addr, vec![coin(amount, NATIVE_DENOM)])
            .unwrap();
    });
}

/// Basic setup for unit test on a single contract
pub fn setup_base_contract() -> (BaseApp, Addr) {
    let mut app: BaseApp =
        BasicAppBuilder::<Empty, Empty>::new_custom().build(|router, _, storage| {
            for player in vec![OWNER_ADDR, PLAYER1_ADDR, PLAYER2_ADDR, PLAYER3_ADDR] {
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(player),
                        vec![coin(9999999999999999999999, NATIVE_DENOM)],
                    )
                    .unwrap();
            }
        });

    let contract = contract_token();
    let code_id = app.store_code(contract);

    let denoms = vec![NATIVE_DENOM.to_string()];
    let init_msg = &InstantiateMsg {
        owner: Addr::unchecked(OWNER_ADDR),
        fee_address: Addr::unchecked(FEE_TREASURY_ADDR),
        fee_percentage: PLATFORM_FEE_RATE as u32,
        min_bet_amount: Uint128::from(MIN_BET_AMOUNT),
        max_bet_amount: Uint128::from(MAX_BET_AMOUNT),
    };

    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(OWNER_ADDR),
            init_msg,
            &[],
            "dice contract",
            Some(OWNER_ADDR.to_string()),
        )
        .unwrap();

    add_balance(&mut app, contract_addr.clone(), MAX_BET_AMOUNT as u128);

    (app, contract_addr)
}
