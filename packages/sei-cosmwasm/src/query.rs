use cosmwasm_std::{Addr, CustomQuery, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::proto_structs::{DenomOracleExchangeRatePair, DexPair, DexTwap, Epoch, OracleTwap};
use crate::route::SeiRoute;
use crate::sei_types::{DenomAuthorityMetadata, OrderResponse};
use crate::Order;

/// SeiQueryWrapper is an override of QueryRequest::Custom to access Sei-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SeiQueryWrapper {
    pub route: SeiRoute,
    pub query_data: SeiQuery,
}

// implement custom query
impl CustomQuery for SeiQueryWrapper {}

/// SeiQuery is defines available query datas
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SeiQuery {
    ExchangeRates {},
    OracleTwaps {
        lookback_seconds: u64,
    },
    DexTwaps {
        contract_address: Addr,
        lookback_seconds: u64,
    },
    Epoch {},
    GetOrders {
        contract_address: Addr,
        account: Addr,
    },
    GetOrderById {
        contract_address: Addr,
        price_denom: String,
        asset_denom: String,
        id: u64,
    },
    GetLatestPrice {
        contract_address: Addr,
        price_denom: String,
        asset_denom: String,
    },
    OrderSimulation {
        contract_address: Addr,
        order: Order,
    },
    DenomAuthorityMetadata {
        denom: String,
    },
    DenomsFromCreator {
        creator: Addr,
    },
    StaticCall {
        from: String,
        to: String,
        data: String, // base64
    },
    Erc20TransferPayload {
        recipient: String,
        amount: Uint128,
    },
    Erc20TransferFromPayload {
        owner: String,
        recipient: String,
        amount: Uint128,
    },
    Erc20ApprovePayload {
        spender: String,
        amount: Uint128,
    },
    Erc20Allowance {
        contract_address: String,
        owner: String,
        spender: String,
    },
    Erc20TokenInfo {
        contract_address: String,
        caller: String,
    },
    Erc20Balance {
        contract_address: String,
        account: String,
    },
    Erc721TransferPayload {
        from: String,
        recipient: String,
        token_id: String,
    },
    Erc721ApprovePayload {
        spender: String,
        token_id: String,
    },
    Erc721Owner {
        caller: String,
        contract_address: String,
        token_id: String,
    },
    Erc721Approved {
        caller: String,
        contract_address: String,
        token_id: String,
    },
    Erc721IsApprovedForAll {
        caller: String,
        contract_address: String,
        owner: String,
        operator: String,
    },
    Erc721SetApprovalAllPayload {
        to: String,
        approved: bool,
    },
    Erc721NameSymbol {
        caller: String,
        contract_address: String,
    },
    Erc721Uri {
        caller: String,
        contract_address: String,
        token_id: String,
    },
}

/// ExchangeRatesResponse is data format returned from OracleRequest::ExchangeRates query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRatesResponse {
    pub denom_oracle_exchange_rate_pairs: Vec<DenomOracleExchangeRatePair>,
}

/// OracleTwapsResponse is data format returned from OracleRequest::OracleTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleTwapsResponse {
    pub oracle_twaps: Vec<OracleTwap>,
}

/// DexTwapsResponse is data format returned from DexTwaps query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DexTwapsResponse {
    pub twaps: Vec<DexTwap>,
}

/// EpochResponse is data format returned from Epoch query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochResponse {
    pub epoch: Epoch,
}

/// GetOrdersResponse is data format returned from GetOrders query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetOrdersResponse {
    pub orders: Vec<OrderResponse>,
}

/// GetOrderdByIdResponse is data format returned from GetOrderById query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetOrderByIdResponse {
    pub order: OrderResponse,
}

/// PriceResponse is data format for a price of an asset pair
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PriceResponse {
    pub snapshot_timestamp_in_seconds: u64,
    pub price: Decimal,
    pub pair: DexPair,
}

/// GetLatestPriceResponse is data format returned from GetLatestPrice query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetLatestPriceResponse {
    pub price: PriceResponse,
}

/// OrderSimulationResponse is data format returned from OrderSimulation query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderSimulationResponse {
    pub executed_quantity: Decimal,
}

/// DenomAuthorityMetadataResponse is data format returned from DenomAuthorityMetadata query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomAuthorityMetadataResponse {
    pub authority_metadata: DenomAuthorityMetadata,
}

/// DenomsFromCreatorResponse is data format returned from DenomsFromCreator query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomsFromCreatorResponse {
    pub denoms: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StaticCallResponse {
    pub data: String, // base64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ErcPayloadResponse {
    pub encoded_payload: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc20AllowanceResponse {
    pub allowance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721OwnerResponse {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721ApprovedResponse {
    pub approved: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721IsApprovedForAllResponse {
    pub is_approved: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721NameSymbolResponse {
    pub name: String,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721UriResponse {
    pub uri: String,
}
