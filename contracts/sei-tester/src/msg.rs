use schemars::JsonSchema;
use sei_cosmwasm::Order;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    PlaceOrders {},
    CancelOrders { order_ids: Vec<u64> },
    CreateDenom {},
    Mint {},
    Burn {},
    ChangeAdmin {},
    SetMetadata {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ExchangeRates {},
    OracleTwaps {
        lookback_seconds: u64,
    },
    DexTwaps {
        contract_address: String,
        lookback_seconds: u64,
    },
    OrderSimulation {
        order: Order,
        contract_address: String,
    },
    Epoch {},
    GetOrders {
        contract_address: String,
        account: String,
    },
    GetOrderById {
        contract_address: String,
        price_denom: String,
        asset_denom: String,
        id: u64,
    },
    GetLatestPrice {
        contract_address: String,
        price_denom: String,
        asset_denom: String,
    },
}
