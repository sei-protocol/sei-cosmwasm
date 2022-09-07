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
    Fokmarket = 3, // fill-or-kill market order
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
    pub data: String, // serialized order data, defined by the specific target contract
    pub status_description: String,
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

// The following are the types used in the sudo response for finalize block
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractOrderResult {
    pub contract_address: String,
    pub order_placement_results: Vec<OrderPlacementResult>,
    pub order_execution_results: Vec<OrderExecutionResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderPlacementResult {
    pub order_id: u64,
    pub status_code: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderExecutionResult {
    pub order_id: u64,
    pub execution_price: Decimal,
    pub executed_quantity: Decimal,
    pub total_notional: Decimal,
    pub position_direction: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MsgPlaceOrdersResponse {
    pub order_ids: Vec<u64>,
}
