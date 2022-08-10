use cosmwasm_std::{Addr, CustomQuery, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proto_structs::{DenomOracleExchangeRatePair, DexTwap, Epoch, OracleTwap};
use crate::route::SeiRoute;
use crate::sei_types::OrderResponse;
use crate::Order;

/// SeiQueryWrapper is an override of QueryRequest::Custom to access Sei-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SeiQueryWrapper {
    pub route: SeiRoute,
    pub query_data: SeiQuery,
}

// implement custom query
impl CustomQuery for SeiQueryWrapper {}

/// SeiQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiQuery {
    ExchangeRates {},
    OracleTwaps {
        lookback_seconds: i64,
    },
    DexTwaps {
        contract_address: Addr,
        lookback_seconds: u64,
    },
    Epoch {},
    GetOrders {
        contract_address: Addr,
        account: Addr,
    },
    GetOrderById {
        contract_address: Addr,
        price_denom: String,
        asset_denom: String,
        id: u64,
    },
    OrderSimulation {
        contract_address: Addr,
        order: Order,
    },
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub denom_oracle_exchange_rate_pairs: Vec<DenomOracleExchangeRatePair>,
}

/// OracleTwapsResponse is data format returned from OracleRequest::OracleTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTwapsResponse {
    pub oracle_twaps: Vec<OracleTwap>,
}

/// DexTwapsResponse is data format returned from DexTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexTwapsResponse {
    pub twaps: Vec<DexTwap>,
}

/// EpochResponse is data format returned from Epoch query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochResponse {
    pub epoch: Epoch,
}

/// GetOrdersResponse is data format returned from GetOrders query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetOrdersResponse {
    pub orders: Vec<OrderResponse>,
}

/// GetOrderdByIdResponse is data format returned from GetOrderById query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetOrderByIdResponse {
    pub order: OrderResponse,
}

/// OrderSimulationResponse is data format returned from OrderSimulation query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderSimulationResponse {
    pub executed_quantity: Decimal,
}
