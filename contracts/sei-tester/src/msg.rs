use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ExchangeRates {},
    OracleTwaps {
        lookback_seconds: i64,
    },
    DexTwaps {
        contract_address: String,
        lookback_seconds: Uint64,
    },
    Epoch {},
}
