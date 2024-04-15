use anyhow::Result as AnyResult;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Api, BankMsg, Binary, BlockInfo, Coin, CosmosMsg, CustomQuery,
    Decimal, Querier, Storage, Uint128, Uint64,
};
use cw_multi_test::{AppResponse, BankSudo, CosmosRouter, Module, SudoMsg};
use schemars::JsonSchema;
use sei_cosmwasm::{
    Cancellation, DenomOracleExchangeRatePair, DexPair, DexTwap, DexTwapsResponse, Epoch,
    EpochResponse, EvmAddressResponse, ExchangeRatesResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwap, OracleTwapsResponse, Order, OrderResponse,
    OrderSimulationResponse, OrderStatus, PositionDirection, SeiAddressResponse, SeiMsg, SeiQuery,
    SeiQueryWrapper, SudoMsg as SeiSudoMsg, StaticCallResponse
};
use serde::de::DeserializeOwned;
use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Div, Mul, Sub},
};

pub struct SeiModule {
    epoch: Epoch,
    exchange_rates: HashMap<String, Vec<DenomOracleExchangeRatePair>>,
}

const GENESIS_EPOCH: Epoch = Epoch {
    genesis_time: String::new(),
    duration: 60,
    current_epoch: 1,
    current_epoch_start_time: String::new(),
    current_epoch_height: 1,
};

pub const EVM_ADDRESS: &str = "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B";
pub const SEI_ADDRESS: &str = "sei1vzxkv3lxccnttr9rs0002s93sgw72h7ghukuhs";

impl SeiModule {
    pub fn new() -> Self {
        SeiModule {
            epoch: GENESIS_EPOCH,
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
            epoch: GENESIS_EPOCH,
            exchange_rates: exchange_rates,
        }
    }

    pub fn set_epoch(&self, new_epoch: Epoch) -> Self {
        SeiModule {
            epoch: new_epoch,
            exchange_rates: (&self.exchange_rates).clone(),
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
    type SudoT = SeiSudoMsg;

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
                return execute_place_orders_helper(
                    storage,
                    block,
                    orders,
                    funds,
                    contract_address,
                );
            }
            SeiMsg::CancelOrders {
                cancellations,
                contract_address,
            } => {
                return execute_cancel_orders_helper(storage, cancellations, contract_address);
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
        block: &BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<Binary> {
        match request.query_data {
            SeiQuery::ExchangeRates {} => Ok(to_json_binary(&get_exchange_rates(
                self.exchange_rates.clone(),
            ))?),
            SeiQuery::OracleTwaps { lookback_seconds } => Ok(to_json_binary(&get_oracle_twaps(
                block,
                self.exchange_rates.clone(),
                lookback_seconds,
            ))?),
            SeiQuery::DexTwaps {
                contract_address,
                lookback_seconds,
            } => Ok(to_json_binary(&get_dex_twaps(
                storage,
                block,
                contract_address,
                lookback_seconds,
            ))?),
            SeiQuery::OrderSimulation {
                order,
                contract_address,
            } => Ok(to_json_binary(&get_order_simulation(
                storage,
                order,
                contract_address,
            ))?),
            SeiQuery::Epoch {} => return query_get_epoch_helper(self.epoch.clone()),
            SeiQuery::GetOrders {
                contract_address,
                account,
            } => {
                return query_get_orders_helper(storage, contract_address, account);
            }
            // TODO: Implement Set and Get latest price in integration tests
            SeiQuery::GetLatestPrice { .. } => {
                panic!("Get Latest Price Query not implemented")
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
            SeiQuery::StaticCall { .. } => {
                Ok(to_json_binary(&StaticCallResponse {
                    data: "static call response".to_string(),
                })?)
            }
            SeiQuery::GetEvmAddress { sei_address } => {
                Ok(to_json_binary(&get_evm_address(sei_address))?)
            }
            SeiQuery::GetSeiAddress { evm_address } => {
                Ok(to_json_binary(&get_sei_address(evm_address))?)
            }
            // TODO: Implement get denom authority metadata in integration tests
            SeiQuery::DenomAuthorityMetadata { .. } => {
                panic!("Denom Authority Metadata not implemented")
            }
            // TODO: Implement get denom from creator in integration tests
            SeiQuery::DenomsFromCreator { .. } => {
                panic!("Denoms From Creator not implemented")
            }
            _ => panic!("Unexpected custom query msg"),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            SeiSudoMsg::Settlement {
                epoch: _,
                entries: _,
            } => Ok(AppResponse {
                events: vec![],
                data: None,
            }),
            SeiSudoMsg::BulkOrderPlacements {
                orders: _,
                deposits: _,
            } => Ok(AppResponse {
                events: vec![],
                data: None,
            }),
            SeiSudoMsg::BulkOrderCancellations { ids: _ } => Ok(AppResponse {
                events: vec![],
                data: None,
            }),
        }
    }
}

// Helper functions

// Dex Module Msg

// Execute: PlaceOrders()
fn execute_place_orders_helper(
    storage: &mut dyn Storage,
    block: &BlockInfo,
    orders: Vec<Order>,
    _funds: Vec<Coin>,
    contract_address: Addr,
) -> AnyResult<AppResponse> {
    // Storage:
    // OrderIdCounter -> OrderId
    // contract_address + "-" + OrderResponses -> OrderResponse[]
    // contract_address + "-" + OrderResponseById + "-" + Price Denom + "-" + Asset Denom + "-" + OrderId -> OrderResponse
    // "OrderTimestamp-" + OrderId -> OrderTimestamp

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
            account: "test account".to_string(),
            contract_address: "test contract".to_string(),
            status_description: "desc".to_string(),
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
        storage.set(
            format!("OrderTimestamp-{}", latest_order_id).as_bytes(),
            &block.time.seconds().to_be_bytes(),
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
        data: Some(to_json_binary(&contract_address).unwrap()),
    })
}

// Execute: CancelOrders()
fn execute_cancel_orders_helper(
    storage: &mut dyn Storage,
    cancellations: Vec<Cancellation>,
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

    let order_ids: Vec<u64> = cancellations.iter().map(|c| -> u64 { c.id }).collect();
    for order_id in order_ids.clone() {
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
        data: Some(to_json_binary(&contract_address).unwrap()),
    })
}

// Oracle Module

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
    block: &BlockInfo,
    rates: HashMap<String, Vec<DenomOracleExchangeRatePair>>,
    lookback_seconds: u64,
) -> OracleTwapsResponse {
    let mut oracle_twaps: Vec<OracleTwap> = Vec::new();
    let lbs = lookback_seconds as u64;

    for key in rates.keys() {
        let pair_rates = rates.get(key).unwrap();
        let mut sum = Decimal::zero();
        let start: u64 = block.time.seconds();
        let mut time: u64 = block.time.seconds();
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

fn get_dex_twaps(
    storage: &dyn Storage,
    block: &BlockInfo,
    contract_address: Addr,
    lookback_seconds: u64,
) -> DexTwapsResponse {
    let mut dex_twaps: HashMap<(String, String), Decimal> = HashMap::new();
    let mut prev_time = block.time.seconds();

    let order_response: GetOrdersResponse = from_json(
        &query_get_orders_helper(storage, contract_address, Addr::unchecked("")).unwrap(),
    )
    .unwrap();

    let mut orders = order_response.orders.clone();
    orders.sort_by(|a, b| b.id.cmp(&a.id));

    for order in orders {
        let timestamp = u64::from_be_bytes(
            storage
                .get(format!("OrderTimestamp-{}", order.id).as_bytes())
                .unwrap()
                .try_into()
                .unwrap(),
        );

        let mut update_fn = |time: u64| {
            if !dex_twaps.contains_key(&(order.asset_denom.clone(), order.price_denom.clone())) {
                dex_twaps.insert(
                    (order.asset_denom.clone(), order.price_denom.clone()),
                    Decimal::zero(),
                );
            }

            let sum = dex_twaps
                .get(&(order.asset_denom.clone(), order.price_denom.clone()))
                .unwrap();

            let new_sum = sum.add(order.price.mul(Decimal::from_ratio(time, 1u64)));

            dex_twaps.insert(
                (order.asset_denom.clone(), order.price_denom.clone()),
                new_sum,
            );
        };

        if block.time.seconds() - timestamp >= lookback_seconds {
            update_fn(lookback_seconds - (block.time.seconds() - prev_time));
            prev_time = timestamp;
        } else if block.time.seconds() - prev_time < lookback_seconds {
            update_fn(prev_time - timestamp);
            prev_time = timestamp;
        }
    }

    let mut twaps: Vec<DexTwap> = Vec::new();
    for key in dex_twaps.keys() {
        let sum = dex_twaps.get(key).unwrap();
        twaps.push(DexTwap {
            pair: DexPair {
                asset_denom: key.0.clone(),
                price_denom: key.1.clone(),
                price_tick_size: Decimal::from_ratio(1u128, 10000u128),
                quantity_tick_size: Decimal::from_ratio(1u128, 10000u128),
            },
            twap: sum.div(Decimal::from_ratio(lookback_seconds, 1u64)),
            lookback_seconds: lookback_seconds,
        });
    }

    DexTwapsResponse { twaps }
}

fn get_order_simulation(
    storage: &dyn Storage,
    order: Order,
    contract_address: Addr,
) -> OrderSimulationResponse {
    let mut executed_quantity = Decimal::zero();

    let orders: GetOrdersResponse = from_json(
        &query_get_orders_helper(storage, contract_address, Addr::unchecked("")).unwrap(),
    )
    .unwrap();

    let valid_orders = if order.position_direction == PositionDirection::Long {
        PositionDirection::Short
    } else {
        PositionDirection::Long
    };

    for order_response in orders.orders {
        if order_response.position_direction == valid_orders {
            if (order_response.position_direction == PositionDirection::Long
                && order.price <= order_response.price)
                || (order_response.position_direction == PositionDirection::Short
                    && order.price >= order_response.price)
            {
                executed_quantity += order_response.quantity;
            }
        }
    }

    OrderSimulationResponse {
        executed_quantity: if executed_quantity > order.quantity {
            order.quantity
        } else {
            executed_quantity
        },
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

    return Ok(to_json_binary(&GetOrdersResponse {
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

    return Ok(to_json_binary(&GetOrderByIdResponse {
        order: order_response,
    })?);
}

// Epoch Module Queries

fn get_epoch(epoch: Epoch) -> EpochResponse {
    EpochResponse { epoch: epoch }
}

fn get_static_call_response() -> StaticCallResponse {
    StaticCallResponse {
        data: "static call response".to_string(),
    }
}
fn get_evm_address(sei_address: String) -> EvmAddressResponse {
    let (evm_address, associated) = match sei_address.as_str() {
        SEI_ADDRESS => (EVM_ADDRESS.to_string(), true),
        _ => (String::new(), false), // default case
    };

    EvmAddressResponse {
        evm_address,
        associated,
    }
}

fn get_sei_address(evm_address: String) -> SeiAddressResponse {
    let (sei_address, associated) = match evm_address.as_str() {
        EVM_ADDRESS => (SEI_ADDRESS.to_string(), true),
        _ => (String::new(), false), // default case
    };

    SeiAddressResponse {
        sei_address,
        associated,
    }
}

// Query: GetEpoch()
fn query_get_epoch_helper(epoch: Epoch) -> AnyResult<Binary> {
    return Ok(to_json_binary(&get_epoch(epoch))?);
}

// TokenFactory Msg

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
        data: Some(to_json_binary(&denom).unwrap()),
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
