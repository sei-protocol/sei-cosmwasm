use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// example of a contract specific order data struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub leverage: Decimal,
    pub position_effect: PositionEffect,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema, Eq, Hash)]
pub enum PositionEffect {
    Unknown,
    Open,
    Close,
}
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
