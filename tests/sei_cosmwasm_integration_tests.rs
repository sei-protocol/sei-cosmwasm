use common::SeiModule;
use common::{get_balance, mock_app};
use cosmwasm_std::{
    coin, from_binary,
    testing::{MockApi, MockStorage},
    Addr, Api, BalanceResponse, Coin, CosmosMsg, Storage, Uint128,
};
use cw_multi_test::{
    App, BankKeeper, ContractWrapper, Executor, FailingDistribution, FailingStaking, Router,
    WasmKeeper,
};
use sei_cosmwasm::{EpochResponse, SeiMsg, SeiQueryWrapper};
use sei_tester::{
    contract::{execute, instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};

mod common;

const ADMIN: &str = "admin";
const NATIVE_DENOM: &str = "usei";

/// Init balances via bank

fn init_default_balances(
    router: &mut Router<
        BankKeeper,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        FailingStaking,
        FailingDistribution,
    >,
    _api: &dyn Api,
    storage: &mut dyn Storage,
) {
    router
        .bank
        .init_balance(
            storage,
            &Addr::unchecked(ADMIN),
            vec![coin(1_000_000_000_000_000, NATIVE_DENOM.to_string())],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &Addr::unchecked("alice"),
            vec![
                coin(10_000_000, "usei".to_string()),
                coin(10_000_000, "uatom".to_string()),
            ],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &Addr::unchecked("bob"),
            vec![
                coin(10_000_000, "usei".to_string()),
                coin(10_000_000, "uatom".to_string()),
            ],
        )
        .unwrap();

    router
        .bank
        .init_balance(
            storage,
            &Addr::unchecked("charlie"),
            vec![
                coin(10_000_000, "usei".to_string()),
                coin(10_000_000, "uatom".to_string()),
            ],
        )
        .unwrap();
}

/// Helper for setting up test

fn setup_test(
    app: &mut App<
        BankKeeper,
        MockApi,
        MockStorage,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        FailingStaking,
        FailingDistribution,
    >,
) -> Addr {
    let sei_tester_code = app.store_code(Box::new(
        ContractWrapper::new(execute, instantiate, query).with_reply(sei_tester::contract::reply),
    ));

    let sei_tester_addr = app
        .instantiate_contract(
            sei_tester_code,
            Addr::unchecked(ADMIN),
            &InstantiateMsg {},
            &[],
            "sei_tester",
            Some(ADMIN.to_string()),
        )
        .unwrap();

    sei_tester_addr
}

/// Basic msg examples

/// Token Factory
#[test]
fn test_tokenfactory_integration_foundation() {
    let mut app = mock_app(init_default_balances);
    setup_test(&mut app);

    let arr = app
        .execute_multi(
            Addr::unchecked(ADMIN),
            vec![CosmosMsg::Custom(SeiMsg::CreateDenom {
                subdenom: "test".to_string(),
            })],
        )
        .unwrap();
    let res = arr.first().unwrap().clone().data;
    let data = res.unwrap();

    let out: String = from_binary(&data).unwrap();
    assert_eq!(out.to_string(), "factory/admin/test");

    app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::MintTokens {
            amount: Coin {
                denom: out.to_string(),
                amount: Uint128::new(1),
            },
        })],
    )
    .unwrap();

    let res: BalanceResponse = get_balance(&app, ADMIN.to_string(), out.to_string());
    assert_eq!(res.amount.amount, Uint128::new(1));
    assert_eq!(res.amount.denom, out);

    let err = app
        .execute_multi(
            Addr::unchecked("fake"),
            vec![CosmosMsg::Custom(SeiMsg::MintTokens {
                amount: Coin {
                    denom: out.to_string(),
                    amount: Uint128::new(1),
                },
            })],
        )
        .err()
        .unwrap();

    assert_eq!(
        err.to_string(),
        "Must be owner of coin factory denom to mint".to_string()
    );

    app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::BurnTokens {
            amount: Coin {
                denom: out.to_string(),
                amount: Uint128::new(1),
            },
        })],
    )
    .unwrap();

    let res: BalanceResponse = get_balance(&app, ADMIN.to_string(), out.to_string());
    assert_eq!(res.amount.amount, Uint128::new(0));
    assert_eq!(res.amount.denom, out);

    let err = app
        .execute_multi(
            Addr::unchecked("fake"),
            vec![CosmosMsg::Custom(SeiMsg::BurnTokens {
                amount: Coin {
                    denom: out.to_string(),
                    amount: Uint128::new(1),
                },
            })],
        )
        .err()
        .unwrap();

    assert_eq!(
        err.to_string(),
        "Must be owner of coin factory denom to burn".to_string()
    );
}

/// Basic querying examples

/// Epoch
#[test]
fn test_epoch_query() {
    let mut app = mock_app(init_default_balances);
    let sei_tester_addr = setup_test(&mut app);

    // Query auction and assert values are what is expected
    let res: EpochResponse = app
        .wrap()
        .query_wasm_smart(sei_tester_addr.clone(), &QueryMsg::Epoch {})
        .unwrap();

    assert_eq!(
        res.epoch.genesis_time,
        "2022-09-15T15:53:04.303018Z".to_string()
    );
    assert_eq!(res.epoch.duration, 61);
    assert_eq!(res.epoch.current_epoch, 1);
    assert_eq!(
        res.epoch.current_epoch_start_time,
        "2022-09-15T15:53:04.303018Z".to_string()
    );
    assert_eq!(res.epoch.current_epoch_height, 1);
}
