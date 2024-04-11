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

    /// Query to for static call to EVM contract.
    /// StaticCall executes the contract associated deployed at `to` address with the given `data`
    /// as parameters while disallowing any modifications to the state during the call.
    StaticCall {
        /// Sei (bech32) address calling the contract
        from: String,
        /// The address of the EVM contract to call
        to: String,
        /// Base64 encoded data to pass to the contract
        data: String, // base64
    },
    /// Query to get hex payload for the ERC-20 `transfer` function
    Erc20TransferPayload {
        /// Sei recipient address
        recipient: String,
        /// Amount to transfer
        amount: Uint128,
    },
    /// Query to get hex payload for the ERC-20 `transferFrom` function
    Erc20TransferFromPayload {
        /// Sei owner address
        owner: String,
        /// Sei recipient address
        recipient: String,
        /// Amount to transfer
        amount: Uint128,
    },
    /// Query to get hex payload for the ERC-20 `approve` function
    Erc20ApprovePayload {
        /// Sei spender address
        spender: String,
        /// Amount to approve
        amount: Uint128,
    },
    /// Query to get the remaining number of tokens that spender will be allowed to spend on behalf
    /// of owner through
    Erc20Allowance {
        /// ERC-20 contract address
        contract_address: String,
        /// Owner Sei address
        owner: String,
        /// Spender Sei address
        spender: String,
    },
    /// Query to get the token info, including the name, symbol, decimals and total supply
    Erc20TokenInfo {
        /// ERC-20 contract address
        contract_address: String,
        /// Caller Sei address
        caller: String,
    },
    /// Query to get the balance of the account with the given address.
    /// Executes the `balanceOf` ERC-20 function under the hood.
    Erc20Balance {
        /// ERC-20 contract address
        contract_address: String,
        /// Account Sei address
        account: String,
    },
    /// Query to get the hex payload for the ERC-721 `transferFrom` function
    Erc721TransferPayload {
        /// Sei address of the sender
        from: String,
        /// Sei address of the recipient
        recipient: String,
        /// The identifier for an NFT. String representation of the token ID
        token_id: String,
    },
    /// Query to get the hex payload for the ERC-721 `approve` function
    Erc721ApprovePayload {
        /// Sei address of the spender
        spender: String,
        /// The identifier for an NFT. String representation of the token ID
        token_id: String,
    },
    /// Query to get the address of the owner of the NFT.
    /// Executes ERC-721 `ownerOf` function under the hood.
    Erc721Owner {
        /// caller Sei address
        caller: String,
        /// ERC-721 contract address
        contract_address: String,
        /// The identifier for an NFT. String representation of the token ID
        token_id: String,
    },
    /// Query to get the approved address for a single NFT. Executes ERC-721 `getApproved` function
    Erc721Approved {
        /// caller Sei address
        caller: String,
        /// ERC-721 contract address
        contract_address: String,
        /// The identifier for an NFT. String representation of the token ID
        token_id: String,
    },
    /// Query if an address is an authorized operator for another address. Executes ERC-721
    /// `isApprovedForAll` function.
    Erc721IsApprovedForAll {
        /// caller Sei address
        caller: String,
        /// ERC-721 contract address
        contract_address: String,
        /// The owner of the NFT Sei address
        owner: String,
        /// The operator Sei address that acts on behalf of the owner
        operator: String,
    },
    /// Query to get the hex payload for the ERC-721 `setApprovalForAll` function.
    Erc721SetApprovalAllPayload {
        /// Sei address of the operator
        to: String,
        /// Boolean representing the status to set
        approved: bool,
    },
    /// Query to get the name and symbol of the ERC-721 contract. Executes ERC-721 `name` and
    /// `symbol` functions under the hood.
    Erc721NameSymbol {
        /// caller Sei address
        caller: String,
        /// ERC-721 contract address
        contract_address: String,
    },
    /// Query to get the URI for a given NFT. Executes ERC-721 `tokenURI` function under the hood.
    Erc721Uri {
        /// caller Sei address
        caller: String,
        /// ERC-721 contract address
        contract_address: String,

        token_id: String,
    },
    /// Query to get the EVM address associated with the given SEI address.
    GetEvmAddress {
        sei_address: String,
    },
    /// Query to get the SEI address associated with the given EVM address.
    GetSeiAddress {
        evm_address: String,
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

/// `StaticCallResponse` is a struct that represents a response containing the result of a static
/// call to an EVM contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StaticCallResponse {
    /// The result of the static call to the EVM contract. It's represented as a base64 encoded
    /// string.
    pub data: String, // base64
}

/// `ErcPayloadResponse` is a struct that represents a response containing the encoded payload for
/// payload generation queries.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ErcPayloadResponse {
    /// The hex encoded payload
    pub encoded_payload: String,
}

/// `Erc20AllowanceResponse` is a struct that represents a response containing the remaining number
/// of tokens that spender will be allowed to spend on behalf of owner.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc20AllowanceResponse {
    /// The amount which spender is still allowed to withdraw from owner
    pub allowance: Uint128,
}

/// `Erc721OwnerResponse` is a struct that represents a response containing the address of the
/// owner.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721OwnerResponse {
    /// The address of the owner of the NFT
    pub owner: String,
}

/// `Erc721ApprovedResponse` is a struct that represents a response containing the address of the
/// approved address for a single NFT.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721ApprovedResponse {
    /// The approved address for this NFT, or the zero address if there is none
    pub approved: String,
}

/// `Erc721IsApprovedForAllResponse` is a struct that represents a response containing a boolean
/// value indicating if an address is an authorized operator for another address
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721IsApprovedForAllResponse {
    /// True if `operator` is an approved operator for `owner`, false otherwise
    pub is_approved: bool,
}

/// `Erc721NameSymbolResponse` is a struct that represents a response containing the name and symbol
/// of the ERC-721 contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721NameSymbolResponse {
    /// The name of the ERC-721 contract
    pub name: String,
    /// The symbol of the ERC-721 contract
    pub symbol: String,
}

/// `Erc721UriResponse` is a struct that represents a response containing the URI for a given NFT.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Erc721UriResponse {
    /// The URI for the given NFT
    pub uri: String,
}

/// `EvmAddressResponse` is a struct that represents a response containing an EVM address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EvmAddressResponse {
    /// The 20-byte EVM address associated to Sei address that's derived from the public part of a
    /// public-private key pair. It's represented as a hex string.
    /// Address is empty if the Sei address is not associated with any EVM address.
    pub evm_address: String,

    /// A boolean value indicating whether the EVM address is associated.
    pub associated: bool,
}

/// `SeiAddressResponse` is a struct that represents a response containing a SEI address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SeiAddressResponse {
    /// The SEI address associated to EVM address. Empty if the EVM address is not associated with
    /// any SEI address.
    pub sei_address: String,

    /// A boolean value indicating whether the SEI address is associated to EVM address.
    pub associated: bool,
}
