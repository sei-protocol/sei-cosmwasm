use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
pub enum PositionDirection {
    Long = 0,
    Short = 1,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
pub enum PositionEffect {
    Open = 0,
    Close = 1,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash, JsonSchema)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
    Liquidation = 2, // TODO: check with @codchen if this is correct
}
