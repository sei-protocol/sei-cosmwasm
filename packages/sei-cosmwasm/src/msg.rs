use crate::sei_types::{OrderType, PositionDirection, PositionEffect};
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// implement custom query
impl CustomMsg for SeiMsg {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<SeiMsg> for CosmosMsg<SeiMsg> {
    fn from(original: SeiMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiMsg {
    PlaceOrders {
        creator: Addr,
        orders: Vec<OrderPlacement>,
        contract_address: Addr,
        funds: Vec<Coin>,
    },
    CancelOrders {
        creator: Addr,
        order_ids: Vec<u64>,
        contract_address: Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OrderPlacement {
    pub position_direction: PositionDirection,
    pub price: Decimal,
    pub quantity: Decimal,
    pub price_denom: String,
    pub asset_denom: String,
    pub position_effect: PositionEffect,
    pub order_type: OrderType,
    pub leverage: Decimal,
}
