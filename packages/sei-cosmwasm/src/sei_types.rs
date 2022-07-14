use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
pub enum PositionDirection {
    Long = 0,
    Short = 1,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    Liquidation = 2, // TODO: check with @codchen if this is correct
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    pub price: Decimal,
    pub quantity: Decimal,
    pub price_denom: String,
    pub asset_denom: String,
    pub order_type: i32,
    pub position_direction: i32,
    pub data: String,
}
