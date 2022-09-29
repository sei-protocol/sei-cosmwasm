use common::mock_app;
use common::SeiModule;
use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Addr, Api, BalanceResponse, BlockInfo, Coin, Decimal, Empty, StdError, Storage, Timestamp,
    Uint128, Uint64,
};
use cw_multi_test::{
    App, BankKeeper, Contract, ContractWrapper, Executor, FailingDistribution, FailingStaking,
    Router, WasmKeeper,
};
use sei_cosmwasm::{GetOrdersResponse, Order, OrderResponse, SeiMsg, SeiQueryWrapper};
use sei_cosmwasm::{OrderStatus, OrderType, PositionDirection};
use sei_tester::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

mod common;

const ADMIN: &str = "admin";
const NATIVE_DENOM: &str = "usei";

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

#[test]
fn test_first_example() {
    let mut app = mock_app(init_default_balances);
    let sei_tester_addr = setup_test(&mut app);

    let res = app
        .execute_contract(
            Addr::unchecked("admin"),
            sei_tester_addr.clone(),
            &ExecuteMsg::CreateDenom {},
            &[],
        )
        .unwrap();

    // // Query auction and assert values are what is expected
    // let orderResponse: GetOrdersResponse = app
    //     .wrap()
    //     .query_wasm_smart(
    //         sei_tester_addr.clone(),
    //         &QueryMsg::GetOrders {
    //             contract_address: sei_tester_addr.to_string(),
    //             account: Addr::unchecked("alice").to_string(),
    //         },
    //     )
    //     .unwrap();
    // let order_data = OrderData {
    //     leverage: Decimal::one(),
    //     position_effect: PositionEffect::Open,
    // };

    // let mockOrderResponse = OrderResponse {
    //     id: 0,
    //     status: OrderStatus::Placed,
    //     price: Decimal::from_atomics(120u128, 0).unwrap(),
    //     quantity: Decimal::one(),
    //     price_denom: "sei".to_string(),
    //     asset_denom: "atom".to_string(),
    //     position_direction: PositionDirection::Long,
    //     order_type: OrderType::Limit,
    //     data: serde_json::to_string(&order_data).unwrap(),
    // };

    //assert_eq!(orderResponse.orders[0], mockOrderResponse);
    //assert_eq!(auction.bid_denom, Some("usei".to_string()));
}
