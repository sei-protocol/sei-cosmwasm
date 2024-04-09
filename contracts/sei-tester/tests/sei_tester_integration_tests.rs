use cosmwasm_std::{
    coin, from_json,
    testing::{MockApi, MockStorage},
    Addr, Api, BalanceResponse, Coin, CosmosMsg, Decimal, Empty, GovMsg, IbcMsg, IbcQuery,
    QueryRequest, StdError, Storage, Uint128,
};
use cosmwasm_std::{BlockInfo, Uint64};
use cw_multi_test::{
    App, BankKeeper, ContractWrapper, DistributionKeeper, Executor, FailingModule, Router,
    StakeKeeper, WasmKeeper,
};

use sei_cosmwasm::{
    Cancellation, DenomOracleExchangeRatePair, DexPair, DexTwap, DexTwapsResponse, EpochResponse,
    EvmAddressResponse, ExchangeRatesResponse, GetOrderByIdResponse, GetOrdersResponse,
    OracleExchangeRate, OracleTwapsResponse, Order, OrderSimulationResponse, OrderStatus,
    OrderType, PositionDirection, SeiAddressResponse, SeiMsg, SeiQuery, SeiQueryWrapper, SeiRoute,
};
use sei_integration_tests::{
    helper::{get_balance, mock_app},
    module::{SeiModule, EVM_ADDRESS, SEI_ADDRESS},
};
use sei_tester::{
    contract::{execute, instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};

const ADMIN: &str = "admin";
const NATIVE_DENOM: &str = "usei";

/// Init balances via bank

fn init_default_balances(
    router: &mut Router<
        BankKeeper,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        StakeKeeper,
        DistributionKeeper,
        FailingModule<IbcMsg, IbcQuery, Empty>,
        FailingModule<GovMsg, Empty, Empty>,
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
        StakeKeeper,
        DistributionKeeper,
        FailingModule<IbcMsg, IbcQuery, Empty>,
        FailingModule<GovMsg, Empty, Empty>,
    >,
) -> Addr {
    let sei_tester_code = app.store_code(Box::new(
        ContractWrapper::new(execute, instantiate, query)
            .with_reply(sei_tester::contract::reply)
            .with_sudo(sei_tester::contract::sudo),
    )); //::<SeiMsg, SeiQueryWrapper>

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
    let mut app = mock_app(init_default_balances, vec![]);
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

    let out: String = from_json(&data).unwrap();
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

/// Epoch Module - query
#[test]
fn test_epoch_query() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    // Query auction and assert values are what is expected
    let res: EpochResponse = app
        .wrap()
        .query_wasm_smart(sei_tester_addr.clone(), &QueryMsg::Epoch {})
        .unwrap();

    assert_eq!(res.epoch.genesis_time, "".to_string());
    assert_eq!(res.epoch.duration, 60);
    assert_eq!(res.epoch.current_epoch, 1);
    assert_eq!(res.epoch.current_epoch_start_time, "".to_string());
    assert_eq!(res.epoch.current_epoch_height, 1);
}

/// Dex Module - place and get orders
#[test]
fn test_dex_module_integration_orders() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    // input params: orders, funds, contract_addr
    let mut orders: Vec<Order> = Vec::new();
    let mut funds = Vec::<Coin>::new();
    let contract_addr = "example_contract".to_string();

    // Make order1
    let price = Decimal::raw(100);
    let quantity = Decimal::raw(1000);
    let price_denom = "USDC".to_string();
    let asset_denom = "ATOM".to_string();
    let order_type = OrderType::Market;
    let position_direction = PositionDirection::Long;
    let data = "".to_string();
    let status_description = "order1".to_string();

    let order1: Order = Order {
        price: price,
        quantity: quantity,
        price_denom: price_denom.clone(),
        asset_denom: asset_denom.clone(),
        order_type: order_type,
        position_direction: position_direction,
        data: data, // serialized order data, defined by the specific target contract
        status_description: status_description,
        nominal: Decimal::zero(),
    };
    orders.push(order1);

    // Make order2
    let price2 = Decimal::raw(500);
    let quantity2 = Decimal::raw(5000);
    let price_denom2 = "DAI".to_string();
    let asset_denom2 = "ATOM".to_string();
    let order_type2 = OrderType::Limit;
    let position_direction2 = PositionDirection::Short;
    let data2 = "".to_string();
    let status_description2 = "order2".to_string();

    let order2: Order = Order {
        price: price2,
        quantity: quantity2,
        price_denom: price_denom2.clone(),
        asset_denom: asset_denom2.clone(),
        order_type: order_type2,
        position_direction: position_direction2,
        data: data2, // serialized order data, defined by the specific target contract
        status_description: status_description2,
        nominal: Decimal::zero(),
    };
    orders.push(order2);

    funds.push(Coin {
        denom: "usei".to_string(),
        amount: Uint128::new(10),
    });

    // Msg PlaceOrders() with orders 1 and 2
    let arr = app
        .execute_multi(
            Addr::unchecked(ADMIN),
            vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
                orders: orders,
                funds: funds,
                contract_address: Addr::unchecked(&contract_addr),
            })],
        )
        .unwrap();
    let res = arr.first().unwrap().clone().data;
    let data = res.unwrap();
    let out: String = from_json(&data).unwrap();
    assert_eq!(out.to_string(), contract_addr.to_string());

    // Query GetOrders() after order 1
    let res: GetOrdersResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetOrders {
                contract_address: contract_addr.to_string(),
                account: sei_tester_addr.to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.orders.len(), 2);
    assert_eq!(res.orders[0].id, 0);
    assert_eq!(res.orders[0].status, OrderStatus::Placed);
    assert_eq!(res.orders[0].price, Decimal::raw(100));
    assert_eq!(res.orders[0].quantity, Decimal::raw(1000));
    assert_eq!(res.orders[0].price_denom.clone(), "USDC");
    assert_eq!(res.orders[0].asset_denom.clone(), "ATOM");
    assert_eq!(res.orders[0].order_type, order_type);
    assert_eq!(res.orders[0].position_direction, position_direction);

    assert_eq!(res.orders[1].id, 1);
    assert_eq!(res.orders[1].status, OrderStatus::Placed);
    assert_eq!(res.orders[1].price, Decimal::raw(500));
    assert_eq!(res.orders[1].quantity, Decimal::raw(5000));
    assert_eq!(res.orders[1].price_denom.clone(), "DAI");
    assert_eq!(res.orders[1].asset_denom.clone(), "ATOM");
    assert_eq!(res.orders[1].order_type, order_type2);
    assert_eq!(res.orders[1].position_direction, position_direction2);

    //Query GetOrders for non-existent contract address
    let res: Result<GetOrderByIdResponse, StdError> = app.wrap().query_wasm_smart(
        sei_tester_addr.clone(),
        &QueryMsg::GetOrders {
            contract_address: "fake_contract_addr".to_string(),
            account: sei_tester_addr.to_string(),
        },
    );

    let error = res.err();
    assert!(error.is_some());

    // Query GetOrderById() for order id 0
    let res: GetOrderByIdResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetOrderById {
                contract_address: contract_addr.to_string(),
                price_denom: price_denom.clone(),
                asset_denom: asset_denom.clone(),
                id: 0,
            },
        )
        .unwrap();

    assert_eq!(res.order.id, 0);
    assert_eq!(res.order.status, OrderStatus::Placed);
    assert_eq!(res.order.price, Decimal::raw(100));
    assert_eq!(res.order.quantity, Decimal::raw(1000));
    assert_eq!(res.order.price_denom.clone(), "USDC");
    assert_eq!(res.order.asset_denom.clone(), "ATOM");
    assert_eq!(res.order.order_type, order_type);
    assert_eq!(res.order.position_direction, position_direction);

    // Query GetOrderById for order id 1
    let res: GetOrderByIdResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetOrderById {
                contract_address: contract_addr.to_string(),
                price_denom: price_denom2.clone(),
                asset_denom: asset_denom2.clone(),
                id: 1,
            },
        )
        .unwrap();

    assert_eq!(res.order.id, 1);
    assert_eq!(res.order.status, OrderStatus::Placed);
    assert_eq!(res.order.price, Decimal::raw(500));
    assert_eq!(res.order.quantity, Decimal::raw(5000));
    assert_eq!(res.order.price_denom.clone(), "DAI");
    assert_eq!(res.order.asset_denom.clone(), "ATOM");
    assert_eq!(res.order.order_type, order_type2);
    assert_eq!(res.order.position_direction, position_direction2);

    // Query GetOrderById for order id 2 (doesn't exist)
    let res: Result<GetOrderByIdResponse, StdError> = app.wrap().query_wasm_smart(
        sei_tester_addr.clone(),
        &QueryMsg::GetOrderById {
            contract_address: contract_addr.to_string(),
            price_denom: price_denom2.clone(),
            asset_denom: asset_denom2.clone(),
            id: 2,
        },
    );
    let error = res.err();
    assert!(error.is_some());

    // CancelOrders for a contract address that doesn't exist
    let mut nonexistent_order_ids: Vec<u64> = Vec::new();
    nonexistent_order_ids.push(3);
    let cancellations: Vec<Cancellation> = nonexistent_order_ids
        .iter()
        .map(|id| -> Cancellation {
            Cancellation {
                id: *id,
                contract_address: "test contract".to_string(),
                price: Decimal::zero(),
                price_denom: "pd".to_string(),
                asset_denom: "ad".to_string(),
                order_type: OrderType::Limit,
                position_direction: PositionDirection::Long,
            }
        })
        .collect();
    let res = app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::CancelOrders {
            cancellations: cancellations,
            contract_address: Addr::unchecked("fake_contract_addr".to_string()),
        })],
    );
    let error = res.err();
    assert!(error.is_some());

    // CancelOrders for order id 1
    let mut cancel_order_ids: Vec<u64> = Vec::new();
    cancel_order_ids.push(0);
    let cancellations: Vec<Cancellation> = cancel_order_ids
        .iter()
        .map(|id| -> Cancellation {
            Cancellation {
                id: *id,
                contract_address: "test contract".to_string(),
                price: Decimal::zero(),
                price_denom: "pd".to_string(),
                asset_denom: "ad".to_string(),
                order_type: OrderType::Limit,
                position_direction: PositionDirection::Long,
            }
        })
        .collect();
    let arr = app
        .execute_multi(
            Addr::unchecked(ADMIN),
            vec![CosmosMsg::Custom(SeiMsg::CancelOrders {
                cancellations: cancellations,
                contract_address: Addr::unchecked(&contract_addr),
            })],
        )
        .unwrap();
    let res = arr.first().unwrap().clone().data;
    let data = res.unwrap();
    let out: String = from_json(&data).unwrap();
    assert_eq!(out.to_string(), contract_addr.to_string());

    // Query GetOrders() after order 0 cancelled
    let res: GetOrdersResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetOrders {
                contract_address: contract_addr.to_string(),
                account: sei_tester_addr.to_string(),
            },
        )
        .unwrap();

    assert_eq!(res.orders.len(), 1);
    assert_eq!(res.orders[0].id, 1);
    assert_eq!(res.orders[0].status, OrderStatus::Placed);
    assert_eq!(res.orders[0].price, Decimal::raw(500));
    assert_eq!(res.orders[0].quantity, Decimal::raw(5000));
    assert_eq!(res.orders[0].price_denom.clone(), "DAI");
    assert_eq!(res.orders[0].asset_denom.clone(), "ATOM");
    assert_eq!(res.orders[0].order_type, order_type2);
    assert_eq!(res.orders[0].position_direction, position_direction2);

    // Query GetOrderById for order id 0 (doesn't exist)
    let res: Result<GetOrderByIdResponse, StdError> = app.wrap().query_wasm_smart(
        sei_tester_addr.clone(),
        &QueryMsg::GetOrderById {
            contract_address: contract_addr.to_string(),
            price_denom: price_denom.clone(),
            asset_denom: asset_denom.clone(),
            id: 0,
        },
    );
    let error = res.err();
    assert!(error.is_some());
}

/// Dex Module - query order simulation
#[test]
fn test_dex_module_query_order_simulation() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    let mut orders: Vec<Order> = Vec::new();
    let mut funds = Vec::<Coin>::new();

    // Make order1
    let price = Decimal::raw(100);
    let quantity = Decimal::raw(1000);
    let price_denom = "USDC".to_string();
    let asset_denom = "ATOM".to_string();
    let order_type = OrderType::Market;
    let position_direction = PositionDirection::Long;
    let data = "".to_string();
    let status_description = "order1".to_string();

    let order1: Order = Order {
        price: price,
        quantity: quantity,
        price_denom: price_denom.clone(),
        asset_denom: asset_denom.clone(),
        order_type: order_type,
        position_direction: position_direction,
        data: data, // serialized order data, defined by the specific target contract
        status_description: status_description,
        nominal: Decimal::zero(),
    };
    orders.push(order1);

    // Make order2
    let price2 = Decimal::raw(500);
    let quantity2 = Decimal::raw(5000);
    let price_denom2 = "USDC".to_string();
    let asset_denom2 = "ATOM".to_string();
    let order_type2 = OrderType::Limit;
    let position_direction2 = PositionDirection::Long;
    let data2 = "".to_string();
    let status_description2 = "order2".to_string();

    let order2: Order = Order {
        price: price2,
        quantity: quantity2,
        price_denom: price_denom2.clone(),
        asset_denom: asset_denom2.clone(),
        order_type: order_type2,
        position_direction: position_direction2,
        data: data2, // serialized order data, defined by the specific target contract
        status_description: status_description2,
        nominal: Decimal::zero(),
    };
    orders.push(order2);

    funds.push(Coin {
        denom: "usei".to_string(),
        amount: Uint128::new(10),
    });

    app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
            orders: orders,
            funds: funds,
            contract_address: Addr::unchecked(&sei_tester_addr.to_string()),
        })],
    )
    .unwrap();

    // Test all of sim order can be fulfilled
    let res: OrderSimulationResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::OrderSimulation {
                contract_address: Addr::unchecked(sei_tester_addr.to_string()),
                order: Order {
                    price: Decimal::raw(100),
                    quantity: Decimal::raw(500),
                    price_denom: "USDC".to_string(),
                    asset_denom: "ATOM".to_string(),
                    order_type: OrderType::Limit,
                    position_direction: PositionDirection::Short,
                    data: "".to_string(),
                    status_description: "test_order".to_string(),
                    nominal: Decimal::zero(),
                },
            },
        }))
        .unwrap();

    let expected_order_sim_res = OrderSimulationResponse {
        executed_quantity: Decimal::raw(500),
    };

    assert_eq!(res, expected_order_sim_res);

    // Test part of sim order can be fulfilled
    let res: OrderSimulationResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::OrderSimulation {
                contract_address: Addr::unchecked(sei_tester_addr.to_string()),
                order: Order {
                    price: Decimal::raw(100),
                    quantity: Decimal::raw(10000),
                    price_denom: "USDC".to_string(),
                    asset_denom: "ATOM".to_string(),
                    order_type: OrderType::Limit,
                    position_direction: PositionDirection::Short,
                    data: "".to_string(),
                    status_description: "test_order".to_string(),
                    nominal: Decimal::zero(),
                },
            },
        }))
        .unwrap();

    let expected_order_sim_res = OrderSimulationResponse {
        executed_quantity: Decimal::raw(6000),
    };

    assert_eq!(res, expected_order_sim_res);

    // Test none of sim order can be fulfilled
    let res: OrderSimulationResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::OrderSimulation {
                contract_address: Addr::unchecked(sei_tester_addr.to_string()),
                order: Order {
                    price: Decimal::raw(100),
                    quantity: Decimal::raw(1000),
                    price_denom: "USDC".to_string(),
                    asset_denom: "ATOM".to_string(),
                    order_type: OrderType::Limit,
                    position_direction: PositionDirection::Long,
                    data: "".to_string(),
                    status_description: "test_order".to_string(),
                    nominal: Decimal::zero(),
                },
            },
        }))
        .unwrap();

    let expected_order_sim_res = OrderSimulationResponse {
        executed_quantity: Decimal::raw(0),
    };

    assert_eq!(res, expected_order_sim_res);

    // Test none of sim order can be fulfilled
    let res: OrderSimulationResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::OrderSimulation {
                contract_address: Addr::unchecked(sei_tester_addr.to_string()),
                order: Order {
                    price: Decimal::raw(10000),
                    quantity: Decimal::raw(1000),
                    price_denom: "USDC".to_string(),
                    asset_denom: "ATOM".to_string(),
                    order_type: OrderType::Limit,
                    position_direction: PositionDirection::Short,
                    data: "".to_string(),
                    status_description: "test_order".to_string(),
                    nominal: Decimal::zero(),
                },
            },
        }))
        .unwrap();

    let expected_order_sim_res = OrderSimulationResponse {
        executed_quantity: Decimal::raw(0),
    };

    assert_eq!(res, expected_order_sim_res);
}

/// Oracle Module - set and query exchange rates
#[test]
fn test_oracle_module_query_exchange_rate() {
    let app = mock_app(
        init_default_balances,
        vec![
            DenomOracleExchangeRatePair {
                denom: "uusdc".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(80),
                    last_update: Uint64::zero(),
                    last_update_timestamp: 0,
                },
            },
            DenomOracleExchangeRatePair {
                denom: "usei".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(70),
                    last_update: Uint64::zero(),
                    last_update_timestamp: 0,
                },
            },
            DenomOracleExchangeRatePair {
                denom: "uusdc".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(90),
                    last_update: Uint64::new(1),
                    last_update_timestamp: 0,
                },
            },
        ],
    );

    let res: ExchangeRatesResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::ExchangeRates {},
        }))
        .unwrap();

    for rate in res.denom_oracle_exchange_rate_pairs {
        match rate.denom.as_str() {
            "usei" => {
                assert_eq!(
                    rate.oracle_exchange_rate,
                    OracleExchangeRate {
                        exchange_rate: Decimal::percent(70),
                        last_update: Uint64::zero(),
                        last_update_timestamp: 0,
                    }
                );
            }
            "uusdc" => {
                assert_eq!(
                    rate.oracle_exchange_rate,
                    OracleExchangeRate {
                        exchange_rate: Decimal::percent(90),
                        last_update: Uint64::new(1),
                        last_update_timestamp: 0,
                    }
                );
            }
            _ => panic!("Unexpected denom"),
        }
    }
}

/// Oracle Module - query TWAP rates
#[test]
fn test_oracle_module_query_twaps() {
    let app = mock_app(
        init_default_balances,
        vec![
            DenomOracleExchangeRatePair {
                denom: "uusdc".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(80),
                    last_update: Uint64::new(1_571_797_411),
                    last_update_timestamp: 0,
                },
            },
            DenomOracleExchangeRatePair {
                denom: "usei".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(70),
                    last_update: Uint64::zero(),
                    last_update_timestamp: 0,
                },
            },
            DenomOracleExchangeRatePair {
                denom: "uusdc".to_string(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::percent(90),
                    last_update: Uint64::new(1_571_797_415),
                    last_update_timestamp: 0,
                },
            },
        ],
    );

    let res: OracleTwapsResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::OracleTwaps {
                lookback_seconds: 10,
            },
        }))
        .unwrap();

    for rate in res.oracle_twaps {
        match rate.denom.as_str() {
            "usei" => {
                assert_eq!(rate.twap, Decimal::percent(70),);
            }
            "uusdc" => {
                assert_eq!(rate.twap, Decimal::percent(84),);
            }
            _ => panic!("Unexpected denom"),
        }
    }
}

#[test]
fn test_dex_module_query_dex_twap() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    let mut orders: Vec<Order> = Vec::new();

    // Make order1
    let price = Decimal::raw(100);
    let quantity = Decimal::raw(1000);
    let price_denom = "USDC".to_string();
    let asset_denom = "ATOM".to_string();
    let order_type = OrderType::Market;
    let position_direction = PositionDirection::Long;
    let data = "".to_string();
    let status_description = "order1".to_string();

    let order1: Order = Order {
        price: price,
        quantity: quantity,
        price_denom: price_denom.clone(),
        asset_denom: asset_denom.clone(),
        order_type: order_type,
        position_direction: position_direction,
        data: data, // serialized order data, defined by the specific target contract
        status_description: status_description,
        nominal: Decimal::zero(),
    };
    orders.push(order1);

    app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
            orders: orders,
            funds: vec![Coin {
                denom: "usei".to_string(),
                amount: Uint128::new(10),
            }],
            contract_address: Addr::unchecked(&sei_tester_addr.to_string()),
        })],
    )
    .unwrap();

    app.set_block(BlockInfo {
        height: 2,
        time: app.block_info().time.plus_seconds(5),
        chain_id: "test-chain".to_string(),
    });

    let mut orders: Vec<Order> = Vec::new();

    // Make order2
    let price2 = Decimal::raw(500);
    let quantity2 = Decimal::raw(5000);
    let price_denom2 = "USDC".to_string();
    let asset_denom2 = "ATOM".to_string();
    let order_type2 = OrderType::Limit;
    let position_direction2 = PositionDirection::Long;
    let data2 = "".to_string();
    let status_description2 = "order2".to_string();

    let order2: Order = Order {
        price: price2,
        quantity: quantity2,
        price_denom: price_denom2.clone(),
        asset_denom: asset_denom2.clone(),
        order_type: order_type2,
        position_direction: position_direction2,
        data: data2, // serialized order data, defined by the specific target contract
        status_description: status_description2,
        nominal: Decimal::zero(),
    };
    orders.push(order2);

    app.execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
            orders: orders,
            funds: vec![Coin {
                denom: "usei".to_string(),
                amount: Uint128::new(10),
            }],
            contract_address: Addr::unchecked(&sei_tester_addr.to_string()),
        })],
    )
    .unwrap();

    app.set_block(BlockInfo {
        height: 3,
        time: app.block_info().time.plus_seconds(5),
        chain_id: "test-chain".to_string(),
    });

    let res: DexTwapsResponse = app
        .wrap()
        .query(&QueryRequest::Custom(SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::DexTwaps {
                contract_address: Addr::unchecked(&sei_tester_addr.to_string()),
                lookback_seconds: 6,
            },
        }))
        .unwrap();

    let expected_twap: DexTwapsResponse = DexTwapsResponse {
        twaps: vec![DexTwap {
            pair: DexPair {
                price_denom: "USDC".to_string(),
                asset_denom: "ATOM".to_string(),
                price_tick_size: Decimal::from_ratio(1u128, 10000u128),
                quantity_tick_size: Decimal::from_ratio(1u128, 10000u128),
            },
            twap: Decimal::raw(433),
            lookback_seconds: 6,
        }],
    };

    assert_eq!(res, expected_twap);
}

/// EVM Module - query EVM address
#[test]
fn test_evm_address_query() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    // Test associated EVM address
    let res: EvmAddressResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetEvmAddressBySeiAddress {
                sei_address: SEI_ADDRESS.to_string(),
            },
        )
        .unwrap();

    let expected_res = EvmAddressResponse {
        evm_address: EVM_ADDRESS.to_string(),
        associated: true,
    };
    assert_eq!(res, expected_res);

    // Test non-associated EVM address
    let res: EvmAddressResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetEvmAddressBySeiAddress {
                sei_address: "fake_address".to_string(),
            },
        )
        .unwrap();

    let expected_res = EvmAddressResponse {
        evm_address: String::new(),
        associated: false,
    };
    assert_eq!(res, expected_res);
}

#[test]
fn test_sei_address_query() {
    let mut app = mock_app(init_default_balances, vec![]);
    let sei_tester_addr = setup_test(&mut app);

    // // Test associated SEI address
    let res: SeiAddressResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetSeiAddressByEvmAddress {
                evm_address: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B".to_string(),
            },
        )
        .unwrap();

    let expected_res = SeiAddressResponse {
        sei_address: SEI_ADDRESS.to_string(),
        associated: true,
    };
    assert_eq!(res, expected_res);

    // Test non-associated SEI address
    let res: SeiAddressResponse = app
        .wrap()
        .query_wasm_smart(
            sei_tester_addr.clone(),
            &QueryMsg::GetSeiAddressByEvmAddress {
                evm_address: "0x999999cf1046e68e36E1aA2E0E07105eDDD1f08E".to_string(),
            },
        )
        .unwrap();

    let expected_res = SeiAddressResponse {
        sei_address: String::new(),
        associated: false,
    };
    assert_eq!(res, expected_res);
}
