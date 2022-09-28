use common::mock_app;
use common::SeiModule;
use cosmwasm_std::{
    coin,
    testing::{MockApi, MockStorage},
    Addr, Api, BalanceResponse, BlockInfo, Coin, Decimal, Empty, StdError, Storage, Timestamp,
    Uint128, Uint64,
};
// use cw20_base::contract::{execute, instantiate, query};
use cw_multi_test::{
    App, BankKeeper, Contract, ContractWrapper, Executor, FailingDistribution, FailingStaking,
    Router, WasmKeeper,
};
use sei_cosmwasm::{SeiMsg, SeiQueryWrapper};
use sei_tester::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

mod common;

const ADMIN: &str = "admin";
const NATIVE_DENOM: &str = "usei";
const TIME: Uint64 = Uint64::new(1_571_797_419);

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

// Temp example
pub fn contract_template() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
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
    // Currently debugging this
    let sei_tester_code = app.store_code(Box::new(
        ContractWrapper::new(
            sei_tester::contract::execute,
            sei_tester::contract::instantiate,
            sei_tester::contract::query,
        )
        .with_reply(sei_tester::contract::reply),
    ));

    let sei_tester_addr = app
        .instantiate_contract(
            sei_tester_code,
            Addr::unchecked(ADMIN),
            &sei_tester::msg::InstantiateMsg {},
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

    app.execute_contract(
        Addr::unchecked("admin"),
        sei_tester_addr.clone(),
        &ExecuteMsg::PlaceOrders {},
        &[Coin {
            denom: "usei".to_string(),
            amount: Uint128::new(10_000),
        }],
    )
    .unwrap();
}
