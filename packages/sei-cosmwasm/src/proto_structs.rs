use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Decimal, Uint64};

// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleExchangeRate {
    pub exchange_rate: Decimal,
    pub last_update: Uint64,
}

// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomOracleExchangeRatePair {
    pub denom: String,
    pub oracle_exchange_rate: OracleExchangeRate,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTwap {
    pub denom: String,
    pub twap: Decimal,
    pub lookback_seconds: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexPair {
    pub price_denom: i32, // TODO: change to string after sei changes denom representation
    pub asset_denom: i32, // TODO: change to string after sei changes denom representation
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexTwap {
    pub pair: DexPair,
    pub twap: Decimal,
    pub look_back_seconds: u64,
}

// Epoch is the struct that matches the data format of Epoch in Epoch Response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Epoch {
    pub genesis_time: String, // represented as ISO8601 UTC
    pub duration: u64,        // Represented as nanos
    pub current_epoch: u64,
    pub current_epoch_start_time: String, // represented as ISO8601 UTC
    pub current_epoch_height: i64,
}
