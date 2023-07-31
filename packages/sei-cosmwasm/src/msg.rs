use crate::sei_types::{Cancellation, DepositInfo, Order, SettlementEntry};
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg};
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
        orders: Vec<Order>,
        funds: Vec<Coin>,
        contract_address: Addr,
    },
    CancelOrders {
        cancellations: Vec<Cancellation>,
        contract_address: Addr,
    },
    CreateDenom {
        subdenom: String,
    },
    MintTokens {
        amount: Coin,
    },
    BurnTokens {
        amount: Coin,
    },
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Settlement {
        epoch: i64,
        entries: Vec<SettlementEntry>,
    },
    BulkOrderPlacements {
        orders: Vec<Order>,
        deposits: Vec<DepositInfo>,
    },
    BulkOrderCancellations {
        ids: Vec<u64>,
    },
}
