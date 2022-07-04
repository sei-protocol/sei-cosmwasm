use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::SeiRoute;
use cosmwasm_std::{CustomQuery, Decimal, Uint64};

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
        contract_address: String,
        lookback_seconds: u64,
    },
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleExchangeRate {
    pub exchange_rate: Decimal,
    pub last_update: Uint64,
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomOracleExchangeRatePair {
    pub denom: String,
    pub oracle_exchange_rate: OracleExchangeRate,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub denom_oracle_exchange_rate_pairs: Vec<DenomOracleExchangeRatePair>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTwap {
    denom: String,
    twap: Decimal,
    lookback_seconds: i64,
}

/// OracleTwapsResponse is data format returned from OracleRequest::OracleTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTwapsResponse {
    pub oracle_twaps: Vec<OracleTwap>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexPair {
    price_denom: i32, // TODO: change to string after sei changes denom representation
    asset_denom: i32, // TODO: change to string after sei changes denom representation
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexTwap {
    pair: DexPair,
    twap: Decimal,
    look_back_seconds: u64,
}

/// DexTwapsResponse is data format returned from DexTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexTwapsResponse {
    pub twaps: Vec<DexTwap>,
}
