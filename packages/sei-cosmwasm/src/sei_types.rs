use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[repr(i32)]
pub enum PositionDirection {
    Long = 0,
    Short = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[repr(i32)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    Liquidation = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
#[repr(i32)]
pub enum OrderStatus {
    Placed = 0,
    FailedToPlace = 1,
    Cancelled = 2,
    Fulfilled = 3,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Order {
    pub price: Decimal,
    pub quantity: Decimal,
    pub price_denom: String,
    pub asset_denom: String,
    pub order_type: OrderType,
    pub position_direction: PositionDirection,
    pub data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OrderResponse {
    pub id: u64,
    pub status: OrderStatus,
    pub price: Decimal,
    pub quantity: Decimal,
    pub price_denom: String,
    pub asset_denom: String,
    pub order_type: OrderType,
    pub position_direction: PositionDirection,
    pub data: String,
}
