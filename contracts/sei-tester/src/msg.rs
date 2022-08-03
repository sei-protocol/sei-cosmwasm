use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use sei_cosmwasm::{Order, OrderType, PositionDirection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidationRequest {
    pub requestor: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct LiquidationResponse {
    pub successful_accounts: Vec<String>,
    pub liquidation_orders: Vec<Order>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Settlement {
        epoch: i64,
        entries: Vec<SettlementEntry>,
    },

    NewBlock {
        epoch: i64,
    },

    BulkOrderPlacements {
        orders: Vec<Order>,
        deposits: Vec<DepositInfo>,
    },

    BulkOrderCancellations {
        ids: Vec<u64>,
    },

    Liquidation {
        requests: Vec<LiquidationRequest>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ExchangeRates {},
    OracleTwaps {
        lookback_seconds: i64,
    },
    DexTwaps {
        contract_address: String,
        lookback_seconds: u64,
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
}
