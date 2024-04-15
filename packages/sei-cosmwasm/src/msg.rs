use crate::sei_types::{Cancellation, DepositInfo, Metadata, Order, SettlementEntry};
use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg, Uint128};
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
    SetMetadata {
        metadata: Metadata,
    },
    /// Calls EVM contract deployed  at `to` address with the given `data`.
    /// Calls EVM contract as if the contract's caller called it directly.
    /// Please note that the CW contract has to be in
    /// [allow list](https://github.com/sei-protocol/sei-chain/blob/seiv2/x/evm/types/params.go#L142)
    /// in order to execute delegate call.
    /// The EVM (Solidity) contract `msg.sender` in this case will be the callers address.
    DelegateCallEvm {
        /// The address of the EVM contract to call
        to: String,
        /// Base64 encoded binary data to pass to the contract
        data: String,
    },
    /// Calls EVM contract deployed at `to` address with specified `value` and `data`.
    /// The from address is the contract address of the contract executing the call.
    /// The EVM (Solidity) contract `msg.sender` in this case will be the 32-byte long
    /// [`cosmwasm_std::CanonicalAddr`] of this contract.
    CallEvm {
        /// The amount to send along with the transaction
        value: Uint128,
        /// The address of the EVM contract to call
        to: String,
        /// Base64 encoded binary data to pass to the contract
        data: String,
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
