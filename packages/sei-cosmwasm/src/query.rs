use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::SeiRoute;
use cosmwasm_std::{Coin, CustomQuery, Decimal, Uint128};

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
}

/// ExchangeRateItem is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateItem {
    pub quote_denom: String,
    pub exchange_rate: Decimal,
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub base_denom: String,
    pub exchange_rates: Vec<ExchangeRateItem>,
}
