use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::route::SeiRoute;
use cosmwasm_std::{CosmosMsg, CustomMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// SeiMsgWrapper is an override of CosmosMsg::Custom to show this works and can be extended in the contract
pub struct SeiMsgWrapper {
    pub route: SeiRoute,
    pub msg_data: SeiMsg,
}

// implement custom query
impl CustomMsg for SeiMsgWrapper {}

// this is a helper to be able to return these as CosmosMsg easier
impl From<SeiMsgWrapper> for CosmosMsg<SeiMsgWrapper> {
    fn from(original: SeiMsgWrapper) -> Self {
        CosmosMsg::Custom(original)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiMsg {}
