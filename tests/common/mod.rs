use cosmwasm_std::{
    from_binary,
    testing::{MockApi, MockQuerier, MockStorage},
    to_binary, Addr, Api, BalanceResponse, BankMsg, BankQuery, Binary, BlockInfo, Coin, CosmosMsg,
    CustomQuery, Decimal, Empty, MemoryStorage, Querier, Storage, Timestamp, Uint128, Uint64,
};
use cw_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, BankSudo, CosmosRouter, FailingDistribution,
    FailingStaking, Module, Router, SudoMsg, WasmKeeper,
};
use schemars::JsonSchema;
use sei_cosmwasm::{
    DenomOracleExchangeRatePair, Epoch, EpochResponse, ExchangeRatesResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwap, OracleTwapsResponse, Order, OrderResponse, OrderStatus, SeiMsg,
    SeiQuery, SeiQueryWrapper,
};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Div, Mul, Sub},
};

use anyhow::Result as AnyResult;

pub struct SeiModule {
    exchange_rates: HashMap<String, Vec<DenomOracleExchangeRatePair>>,
}

impl SeiModule {
    pub fn new() -> Self {
        SeiModule {
            exchange_rates: HashMap::new(),
        }
    }

    pub fn new_with_oracle_exchange_rates(rates: Vec<DenomOracleExchangeRatePair>) -> Self {
        let mut exchange_rates: HashMap<String, Vec<DenomOracleExchangeRatePair>> = HashMap::new();

        for rate in rates {
            let arr = exchange_rates
                .entry(rate.denom.clone())
                .or_insert_with(Vec::new);

            match arr.binary_search_by(|x| {
                rate.oracle_exchange_rate
                    .last_update
                    .cmp(&x.oracle_exchange_rate.last_update)
            }) {
                Ok(_) => {}
                Err(pos) => arr.insert(pos, rate.clone()),
            };
        }

        SeiModule {
            exchange_rates: exchange_rates,
        }
    }
}

impl Default for SeiModule {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for SeiModule {
    type ExecT = SeiMsg;
    type QueryT = SeiQueryWrapper;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            SeiMsg::PlaceOrders {
                orders,
                funds,
                contract_address,
            } => {
                return execute_place_orders_helper(storage, orders, funds, contract_address);
            }
            SeiMsg::CancelOrders {
                order_ids,
                contract_address,
            } => {
                return execute_cancel_orders_helper(storage, order_ids, contract_address);
            }
            SeiMsg::CreateDenom { subdenom } => {
                return execute_create_denom_helper(storage, sender, subdenom);
            }
            SeiMsg::MintTokens { amount } => {
                return execute_mint_tokens_helper(api, storage, router, block, sender, amount);
            }
            SeiMsg::BurnTokens { amount } => {
                return execute_burn_tokens_helper(api, storage, router, block, sender, amount);
            }
            _ => panic!("Unexpected custom exec msg"),
        }
    }

    fn query(
        &self,
        _api: &dyn Api,
        storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<Binary> {
        match request.query_data {
            SeiQuery::ExchangeRates {} => {
                Ok(to_binary(&get_exchange_rates(self.exchange_rates.clone()))?)
            }
            SeiQuery::OracleTwaps { lookback_seconds } => Ok(to_binary(&get_oracle_twaps(
                self.exchange_rates.clone(),
                lookback_seconds,
            ))?),
            SeiQuery::DexTwaps {
                contract_address: _,
                lookback_seconds: _,
            } => Ok(Binary::default()),
            SeiQuery::OrderSimulation {
                order: _,
                contract_address: _,
            } => Ok(Binary::default()),
            // TODO: replace with app stored data
            SeiQuery::Epoch {} => Ok(to_binary(&EpochResponse {
                epoch: Epoch {
                    genesis_time: "2022-09-15T15:53:04.303018Z".to_string(),
                    duration: 60,
                    current_epoch: 1,
                    current_epoch_start_time: "2022-09-15T15:53:04.303018Z".to_string(),
                    current_epoch_height: 1,
                },
            })?),
            SeiQuery::GetOrders {
                contract_address,
                account,
            } => {
                return query_get_orders_helper(storage, contract_address, account);
            }
            SeiQuery::GetOrderById {
                contract_address,
                price_denom,
                asset_denom,
                id,
            } => {
                return query_get_order_by_id_helper(
                    storage,
                    contract_address,
                    price_denom,
                    asset_denom,
                    id,
                );
            }
            SeiQuery::GetDenomFeeWhitelist {} => Ok(Binary::default()),
            SeiQuery::CreatorInDenomFeeWhitelist { creator: _ } => Ok(Binary::default()),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        todo!()
    }
}

// Helper functions

// Dex Module

// Execute: PlaceOrders()
fn execute_place_orders_helper(
    storage: &mut dyn Storage,
    orders: Vec<Order>,
    _funds: Vec<Coin>,
    contract_address: Addr,
) -> AnyResult<AppResponse> {
    // Storage:
    // OrderIdCounter -> OrderId
    // contract_address + "-" + OrderResponses -> OrderResponse[]
    // contract_address + "-" + OrdeResponseById + "-" + Price Denom + "-" + Asset Denom + "-" + OrderId -> OrderResponse

    // Get latest order id
    let mut latest_order_id: u64 = 0;
    let curr = storage.get("OrderIdCounter".as_bytes());
    if storage.get("OrderIdCounter".as_bytes()).is_some() {
        latest_order_id = String::from_utf8(curr.unwrap_or_default())
            .unwrap_or_default()
            .parse::<u64>()
            .unwrap();
    }

    // get existing orders
    let order_responses_key = contract_address.to_string() + "-" + "OrderResponses";

    let mut order_responses: Vec<OrderResponse> = Vec::new();
    let existing_order_responses = storage.get(order_responses_key.as_bytes());
    if existing_order_responses.is_some() {
        //let order_responses_key = contract_address.to_string() + "-" + "OrderResponses";
        let responses_json: String =
            serde_json::from_slice(&existing_order_responses.clone().unwrap()).unwrap();
        order_responses = serde_json::from_str(&responses_json).unwrap();
    }
    // Iterate through orders, make OrderResponse
    for order in orders.iter() {
        let order_response = OrderResponse {
            id: latest_order_id,
            status: OrderStatus::Placed,
            price: order.price,
            quantity: order.quantity,
            price_denom: order.price_denom.clone(),
            asset_denom: order.asset_denom.clone(),
            order_type: order.order_type,
            position_direction: order.position_direction,
            data: order.data.clone(),
        };
        order_responses.push(order_response.clone());

        // update GetOrderById() -> OrderResponse storage
        let response_json = serde_json::to_string(&order_response);
        let order_id_key = contract_address.to_string()
            + "-"
            + "OrderResponseById"
            + "-"
            + &order.price_denom.clone()
            + "-"
            + &order.asset_denom.clone()
            + "-"
            + &latest_order_id.to_string();
        storage.set(
            order_id_key.as_bytes(),
            &serde_json::to_vec(&response_json.unwrap_or_default()).unwrap(),
        );

        latest_order_id += 1;
    }

    let responses_json = serde_json::to_string(&order_responses);

    // update GetOrders() -> OrderResponse[] storage
    storage.set(
        order_responses_key.as_bytes(),
        &serde_json::to_vec(&responses_json.unwrap_or_default()).unwrap(),
    );
    // update OrderIdCounter -> latest_order_id storage
    storage.set(
        "OrderIdCounter".as_bytes(),
        latest_order_id.to_string().as_bytes(),
    );

    Ok(AppResponse {
        events: vec![],
        data: Some(to_binary(&contract_address).unwrap()),
    })
}

// Execute: CancelOrders()
fn execute_cancel_orders_helper(
    storage: &mut dyn Storage,
    order_ids: Vec<u64>,
    contract_address: Addr,
) -> AnyResult<AppResponse> {
    // get existing orders
    let order_responses_key = contract_address.to_string() + "-" + "OrderResponses";

    let existing_order_responses = storage.get(order_responses_key.as_bytes());
    if !existing_order_responses.is_some() {
        return Err(anyhow::anyhow!(
            "CancelOrders: orders for contract_address do not exist"
        ));
    }

    let responses_json: String =
        serde_json::from_slice(&existing_order_responses.clone().unwrap()).unwrap();
    let mut order_responses: Vec<OrderResponse> = serde_json::from_str(&responses_json).unwrap();

    for order_id in &order_ids.clone() {
        let order_response: Vec<OrderResponse> = order_responses
            .clone()
            .into_iter()
            .filter(|o| order_id.clone() == o.id)
            .collect();
        let order_id_key = contract_address.to_string()
            + "-"
            + "OrderResponseById"
            + "-"
            + &order_response[0].price_denom.clone()
            + "-"
            + &order_response[0].asset_denom.clone()
            + "-"
            + &order_id.to_string();
        // Remove individual for GetOrderById()
        storage.remove(order_id_key.as_bytes());
    }

    order_responses = order_responses
        .into_iter()
        .filter(|o| !order_ids.contains(&o.id))
        .collect();

    let responses_json = serde_json::to_string(&order_responses);

    // update GetOrders() -> OrderResponse[] storage
    storage.set(
        order_responses_key.as_bytes(),
        &serde_json::to_vec(&responses_json.unwrap_or_default()).unwrap(),
    );

    Ok(AppResponse {
        events: vec![],
        data: Some(to_binary(&contract_address).unwrap()),
    })
}

fn get_exchange_rates(
    rates: HashMap<String, Vec<DenomOracleExchangeRatePair>>,
) -> ExchangeRatesResponse {
    let mut exchange_rates: Vec<DenomOracleExchangeRatePair> = Vec::new();

    for key in rates.keys() {
        let rate = rates.get(key).unwrap();
        exchange_rates.push(rate[0].clone());
    }

    ExchangeRatesResponse {
        denom_oracle_exchange_rate_pairs: exchange_rates,
    }
}

fn get_oracle_twaps(
    rates: HashMap<String, Vec<DenomOracleExchangeRatePair>>,
    lookback_seconds: i64,
) -> OracleTwapsResponse {
    let mut oracle_twaps: Vec<OracleTwap> = Vec::new();
    let lbs = lookback_seconds as u64;

    for key in rates.keys() {
        let pair_rates = rates.get(key).unwrap();
        let mut sum = Decimal::zero();
        let start: u64 = 1_571_797_419;
        let mut time: u64 = 1_571_797_419;
        let mut last_rate = Decimal::zero();

        if pair_rates[0].oracle_exchange_rate.last_update < Uint64::new(start - lbs) {
            oracle_twaps.push(OracleTwap {
                denom: key.clone(),
                twap: pair_rates[0].oracle_exchange_rate.exchange_rate,
                lookback_seconds: lookback_seconds,
            });
            continue;
        }

        // Average prices of rates for the past lookback_seconds
        for rate in pair_rates {
            last_rate = rate.oracle_exchange_rate.exchange_rate;
            if Uint64::new(start) - rate.oracle_exchange_rate.last_update < Uint64::new(lbs) {
                sum += last_rate.mul(Decimal::from_ratio(
                    Uint128::new((time - rate.oracle_exchange_rate.last_update.u64()).into()),
                    Uint128::one(),
                ));
                time = rate.oracle_exchange_rate.last_update.u64();
            } else {
                break;
            }
        }

        if Uint64::new(start - time) < Uint64::new(lbs) {
            let sec: u64 = lbs;
            let diff = sec.sub(start - time);
            sum += last_rate.mul(Decimal::from_ratio(
                Uint128::new(diff.into()),
                Uint128::one(),
            ));
        }

        oracle_twaps.push(OracleTwap {
            denom: key.clone(),
            twap: sum.div(Decimal::from_ratio(
                Uint128::new(lbs.into()),
                Uint128::one(),
            )),
            lookback_seconds: lookback_seconds,
        });
    }

    OracleTwapsResponse {
        oracle_twaps: oracle_twaps,
    }
}

// Query: GetOrders()
fn query_get_orders_helper(
    storage: &dyn Storage,
    contract_address: Addr,
    _account: Addr,
) -> AnyResult<Binary> {
    let order_responses_key = contract_address.to_string() + "-" + "OrderResponses";
    let existing_order_responses = storage.get(order_responses_key.as_bytes());
    if !existing_order_responses.is_some() {
        return Err(anyhow::anyhow!(
            "GetOrders: orders for contract_address do not exist"
        ));
    }
    let responses_json: String =
        serde_json::from_slice(&existing_order_responses.clone().unwrap()).unwrap();

    let order_responses: Vec<OrderResponse> = serde_json::from_str(&responses_json).unwrap();

    return Ok(to_binary(&GetOrdersResponse {
        orders: order_responses,
    })?);
}

// Query: GetOrderById()
fn query_get_order_by_id_helper(
    storage: &dyn Storage,
    contract_address: Addr,
    price_denom: String,
    asset_denom: String,
    id: u64,
) -> AnyResult<Binary> {
    let order_id_key = contract_address.to_string()
        + "-"
        + "OrderResponseById"
        + "-"
        + &price_denom
        + "-"
        + &asset_denom
        + "-"
        + &id.to_string();
    let existing_order_response = storage.get(order_id_key.as_bytes());

    if !existing_order_response.is_some() {
        return Err(anyhow::anyhow!("GetOrderById: order for id does not exist"));
    }

    let response_json: String =
        serde_json::from_slice(&existing_order_response.clone().unwrap()).unwrap();

    let order_response: OrderResponse = serde_json::from_str(&response_json).unwrap();

    return Ok(to_binary(&GetOrderByIdResponse {
        order: order_response,
    })?);
}

// TokenFactory

// Execute: CreateDenom()
fn execute_create_denom_helper(
    storage: &mut dyn Storage,
    sender: Addr,
    subdenom: String,
) -> AnyResult<AppResponse> {
    let denom = format!("factory/{}/{}", sender, subdenom);
    if storage.get(denom.as_bytes()).is_some() {
        return Err(anyhow::anyhow!("denom already exists"));
    }
    storage.set(denom.as_bytes(), sender.to_string().as_bytes());
    Ok(AppResponse {
        events: vec![],
        data: Some(to_binary(&denom).unwrap()),
    })
}

// Execute: MintTokens()
fn execute_mint_tokens_helper<ExecC, QueryC>(
    api: &dyn Api,
    storage: &mut dyn Storage,
    router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
    block: &BlockInfo,
    sender: Addr,
    amount: Coin,
) -> AnyResult<AppResponse>
where
    ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryC: CustomQuery + DeserializeOwned + 'static,
{
    let owner = storage.get(amount.denom.as_bytes());
    if owner.is_none() || owner.unwrap() != sender.to_string().as_bytes() {
        return Err(anyhow::anyhow!(
            "Must be owner of coin factory denom to mint"
        ));
    }
    router.sudo(
        api,
        storage,
        block,
        SudoMsg::Bank(BankSudo::Mint {
            to_address: sender.to_string(),
            amount: vec![amount],
        }),
    )
}

// Execute: BurnTokens()
fn execute_burn_tokens_helper<ExecC, QueryC>(
    api: &dyn Api,
    storage: &mut dyn Storage,
    router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
    block: &BlockInfo,
    sender: Addr,
    amount: Coin,
) -> AnyResult<AppResponse>
where
    ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryC: CustomQuery + DeserializeOwned + 'static,
{
    let owner = storage.get(amount.denom.as_bytes());
    if owner.is_none() || owner.unwrap() != sender.to_string().as_bytes() {
        return Err(anyhow::anyhow!(
            "Must be owner of coin factory denom to burn"
        ));
    }
    Ok(router
        .execute(
            api,
            storage,
            block,
            sender,
            CosmosMsg::Bank(BankMsg::Burn {
                amount: vec![amount],
            }),
        )
        .unwrap())
}

// Get balance
pub fn get_balance(
    app: &App<BankKeeper, MockApi, MemoryStorage, SeiModule, WasmKeeper<SeiMsg, SeiQueryWrapper>>,
    addr: String,
    denom: String,
) -> BalanceResponse {
    let arr = app.read_module(|router, api, storage| {
        router.bank.query(
            api,
            storage,
            &MockQuerier::default(),
            &BlockInfo {
                height: 0,
                time: Timestamp::from_seconds(0u64),
                chain_id: "test".to_string(),
            },
            BankQuery::Balance {
                address: addr,
                denom: denom,
            },
        )
    });
    from_binary(&arr.unwrap()).unwrap()
}

// Mock app
pub fn mock_app<F>(
    init_fn: F,
    rates: Vec<DenomOracleExchangeRatePair>,
) -> App<
    BankKeeper,
    MockApi,
    MockStorage,
    SeiModule,
    WasmKeeper<SeiMsg, SeiQueryWrapper>,
    FailingStaking,
    FailingDistribution,
>
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            SeiModule,
            WasmKeeper<SeiMsg, SeiQueryWrapper>,
            FailingStaking,
            FailingDistribution,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    let appbuilder: AppBuilder<
        BankKeeper,
        MockApi,
        MockStorage,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        FailingStaking,
        FailingDistribution,
    > = AppBuilder::new()
        .with_custom(SeiModule::new_with_oracle_exchange_rates(rates))
        .with_wasm::<SeiModule, WasmKeeper<SeiMsg, SeiQueryWrapper>>(WasmKeeper::new());

    appbuilder.build(init_fn)
}
