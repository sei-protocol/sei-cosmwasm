use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Replicates the cosmos-sdk bank module Metadata type
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct Metadata {
    pub description: String,
    pub denom_units: Vec<DenomUnit>,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DenomAuthorityMetadata {
    pub admin: String,
}

/// Replicates the cosmos-sdk bank module DenomUnit type
#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Vec<String>,
}

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
    Fokmarket = 3,        // fill-or-kill market order
    Fokmarketbyvalue = 4, // fill-or-kill market by value order
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
    pub nominal: Decimal, // only needed for Fokmarketbyvalue order
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Cancellation {
    pub id: u64,
    pub contract_address: String,
    pub price_denom: String,
    pub asset_denom: String,
    pub order_type: OrderType,
    pub position_direction: PositionDirection,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OrderResponse {
    pub id: u64,
    pub status: OrderStatus,
    pub account: String,
    pub contract_address: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub price_denom: String,
    pub asset_denom: String,
    pub order_type: OrderType,
    pub position_direction: PositionDirection,
    pub data: String,
    pub status_description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SettlementEntry {
    pub account: String,
    pub price_denom: String,
    pub asset_denom: String,
    pub quantity: Decimal,
    pub execution_cost_or_proceed: Decimal,
    pub expected_cost_or_proceed: Decimal,
    pub position_direction: PositionDirection,
    pub order_type: OrderType,
    pub order_id: u64,
    pub timestamp: u64,
    pub height: u64,
    pub settlement_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositInfo {
    pub account: String,
    pub denom: String,
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BulkOrderPlacementsResponse {
    pub unsuccessful_orders: Vec<UnsuccessfulOrder>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct UnsuccessfulOrder {
    pub id: u64,
    pub reason: String,
}
