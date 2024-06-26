use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Uint128};
use cw20::{BalanceResponse, TokenInfoResponse};

use crate::query::{
    DenomAuthorityMetadataResponse, DenomsFromCreatorResponse, DexTwapsResponse, EpochResponse,
    Erc20AllowanceResponse, Erc721ApprovedResponse, Erc721IsApprovedForAllResponse,
    Erc721NameSymbolResponse, Erc721OwnerResponse, Erc721UriResponse, ErcPayloadResponse,
    EvmAddressResponse, ExchangeRatesResponse, GetLatestPriceResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwapsResponse, OrderSimulationResponse, SeiAddressResponse, SeiQuery,
    SeiQueryWrapper, StaticCallResponse,
};
use crate::route::SeiRoute;
use crate::Order;

/// This is a helper wrapper to easily use our custom queries
pub struct SeiQuerier<'a> {
    querier: &'a QuerierWrapper<'a, SeiQueryWrapper>,
}

impl<'a> SeiQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<SeiQueryWrapper>) -> Self {
        SeiQuerier { querier }
    }

    /*
    query oracle module
    */
    pub fn query_exchange_rates(&self) -> StdResult<ExchangeRatesResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::ExchangeRates {},
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_oracle_twaps(&self, lookback_seconds: u64) -> StdResult<OracleTwapsResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::OracleTwaps { lookback_seconds },
        }
        .into();

        self.querier.query(&request)
    }

    /*
    query dex module
    */
    pub fn query_dex_twaps(
        &self,
        lookback_seconds: u64,
        contract_address: Addr,
    ) -> StdResult<DexTwapsResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::DexTwaps {
                contract_address,
                lookback_seconds,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_order_simulation(
        &self,
        order: Order,
        contract_address: Addr,
    ) -> StdResult<OrderSimulationResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::OrderSimulation {
                contract_address,
                order,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_get_orders(
        &self,
        contract_address: Addr,
        account: Addr,
    ) -> StdResult<GetOrdersResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::GetOrders {
                contract_address,
                account,
            },
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_get_order_by_id(
        &self,
        contract_address: Addr,
        price_denom: String,
        asset_denom: String,
        order_id: u64,
    ) -> StdResult<GetOrderByIdResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::GetOrderById {
                contract_address,
                price_denom,
                asset_denom,
                id: order_id,
            },
        }
        .into();
        self.querier.query(&request)
    }

    /*
    query epoch module
    */
    pub fn query_epoch(&self) -> StdResult<EpochResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Epoch,
            query_data: SeiQuery::Epoch {},
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_get_latest_price(
        &self,
        contract_address: Addr,
        price_denom: String,
        asset_denom: String,
    ) -> StdResult<GetLatestPriceResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Dex,
            query_data: SeiQuery::GetLatestPrice {
                contract_address,
                price_denom,
                asset_denom,
            },
        }
        .into();
        self.querier.query(&request)
    }

    /*
    query tokenfactory module
    */
    pub fn query_denom_authority_metadata(
        &self,
        denom: String,
    ) -> StdResult<DenomAuthorityMetadataResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Tokenfactory,
            query_data: SeiQuery::DenomAuthorityMetadata { denom },
        }
        .into();
        self.querier.query(&request)
    }

    pub fn query_denoms_from_creator(&self, creator: Addr) -> StdResult<DenomsFromCreatorResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Tokenfactory,
            query_data: SeiQuery::DenomsFromCreator { creator },
        }
        .into();
        self.querier.query(&request)
    }

    /// Calls the EVM contract deployed at the `to` address with the given `data`.
    /// The from address is the caller's Sei native (bech32-encoded 'sei*') address.
    /// Please note that the CW contract has to be in the allow list in order to execute a delegate
    /// call.
    ///
    /// The EVM (Solidity) contract `msg.sender` in this case will be the caller's address.
    ///
    /// # Arguments
    /// * `from` - Sei native (bech32-encoded 'sei*') address of the caller.
    /// * `to` - The address of the EVM contract to call.
    /// * `data` - Base64 encoded data to pass to the contract.
    ///
    /// # Returns
    ///
    /// * `StdResult<StaticCallResponse>` - A standard result that wraps the `StaticCallResponse`
    /// struct.
    ///
    /// # Errors
    /// This function will return an error if the query to the EVM fails.
    pub fn static_call(
        &self,
        from: String,
        to: String,
        data: String,
    ) -> StdResult<StaticCallResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::StaticCall { from, to, data },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get hex payload for the ERC-20 `transfer` function
    ///
    /// # Arguments
    /// * `recipient` - Recipient Sei native (bech32-encoded 'sei*') address.
    /// * `amount` - The amount to transfer.
    ///
    /// # Returns
    ///
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`
    /// struct. The `ErcPayloadResponse` struct contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_transfer_payload(
        &self,
        recipient: String,
        amount: Uint128,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TransferPayload { recipient, amount },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get hex payload for the ERC-20 `transferFrom` function
    ///
    /// # Arguments
    /// * `owner` - Owner Sei native (bech32-encoded 'sei*') address.
    /// * `recipient` - Recipient Sei native (bech32-encoded 'sei*') address.
    /// * `amount` - The amount to transfer.
    ///
    /// # Returns
    ///
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`
    /// struct. The `ErcPayloadResponse` struct contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_transfer_from_payload(
        &self,
        owner: String,
        recipient: String,
        amount: Uint128,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TransferFromPayload {
                owner,
                recipient,
                amount,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get hex payload for the ERC-20 `approve` function
    ///
    /// # Arguments
    /// * `spender` - Spender Sei native (bech32-encoded 'sei*') address.
    /// * `amount` - The amount to approve.
    ///
    /// # Returns
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`
    /// struct. The `ErcPayloadResponse` struct contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_approve_payload(
        &self,
        spender: String,
        amount: Uint128,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20ApprovePayload { spender, amount },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the remaining number of tokens that spender will be allowed to spend on behalf
    ///  of owner through
    ///
    /// # Arguments
    /// * `contract_address` - The contract address of the ERC-20 token.
    /// * `owner` - Owner Sei native (bech32-encoded 'sei*') address.
    /// * `spender` - Spender Sei native (bech32-encoded 'sei*') address.
    ///
    /// # Returns
    /// * `StdResult<Erc20AllowanceResponse>` - A standard result that wraps the
    /// `Erc20AllowanceResponse`. `Erc20AllowanceResponse` contains the amount which spender
    /// is still allowed to withdraw from owner
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_allowance(
        &self,
        contract_address: String,
        owner: String,
        spender: String,
    ) -> StdResult<Erc20AllowanceResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20Allowance {
                contract_address,
                owner,
                spender,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the token info, including the name, symbol, decimals and total supply
    ///
    /// # Arguments
    /// * `contract_address` - The contract address of the ERC-20 token.
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    ///
    /// # Returns
    /// * `StdResult<TokenInfoResponse>` - A standard result that wraps the `TokenInfoResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_token_info(
        &self,
        contract_address: String,
        caller: String,
    ) -> StdResult<TokenInfoResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TokenInfo {
                contract_address,
                caller,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the balance of the account with the given address.
    /// Executes the `balanceOf` ERC-20 function under the hood.
    ///
    /// # Arguments
    /// * `contract_address` - The contract address of the ERC-20 token.
    /// * `account` - Sei native (bech32-encoded 'sei*') account address.
    ///
    /// # Returns
    /// * `StdResult<BalanceResponse>` - A standard result that wraps the `BalanceResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc20_balance(
        &self,
        contract_address: String,
        account: String,
    ) -> StdResult<BalanceResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20Balance {
                contract_address,
                account,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the address of the owner of the NFT.
    /// Executes ERC-721 `ownerOf` function under the hood.
    ///
    /// # Arguments
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    /// * `contract_address` - The contract address of the ERC-721 token.
    /// * `token_id` -  The identifier for an NFT. String representation of the token ID.
    ///
    /// # Returns
    /// * `StdResult<Erc721OwnerResponse>` - A standard result that wraps the `Erc721OwnerResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_owner(
        &self,
        caller: String,
        contract_address: String,
        token_id: String,
    ) -> StdResult<Erc721OwnerResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Owner {
                caller,
                contract_address,
                token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the approved address for a single NFT. Executes ERC-721 `getApproved` function.
    ///
    /// # Arguments
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    /// * `contract_address` - The contract address of the ERC-721 token.
    /// * `token_id` -  The identifier for an NFT. String representation of the token ID.
    ///
    /// # Returns
    /// * `StdResult<Erc721ApprovedResponse>` - A standard result that wraps the
    /// `Erc721ApprovedResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_approved(
        &self,
        caller: String,
        contract_address: String,
        token_id: String,
    ) -> StdResult<Erc721ApprovedResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Approved {
                caller,
                contract_address,
                token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query if an address is an authorized operator for another address. Executes ERC-721
    /// `isApprovedForAll` function.
    ///
    /// # Arguments
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    /// * `contract_address` - The contract address of the ERC-721 token.
    /// * `owner` - The owner of the NFT Sei native (bech32-encoded 'sei*') address
    /// * `operator` - The operator Sei address that acts on behalf of the owner
    ///
    /// # Returns
    /// * `StdResult<Erc721IsApprovedForAllResponse>` - A standard result that wraps the
    /// `Erc721IsApprovedForAllResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_is_approved_for_all(
        &self,
        caller: String,
        contract_address: String,
        owner: String,
        operator: String,
    ) -> StdResult<Erc721IsApprovedForAllResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721IsApprovedForAll {
                caller,
                contract_address,
                owner,
                operator,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the name and symbol of the ERC-721 contract. Executes ERC-721 `name` and
    /// `symbol` functions under the hood.
    ///
    /// # Arguments
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    /// * `contract_address` - The contract address of the ERC-721 token.
    ///
    /// # Returns
    /// * `StdResult<Erc721NameSymbolResponse>` - A standard result that wraps the
    /// `Erc721NameSymbolResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_name_symbol(
        &self,
        caller: String,
        contract_address: String,
    ) -> StdResult<Erc721NameSymbolResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721NameSymbol {
                caller,
                contract_address,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the URI for a given NFT. Executes ERC-721 `tokenURI` function under the hood.
    ///
    /// # Arguments
    /// * `caller` - Caller Sei native (bech32-encoded 'sei*') address.
    /// * `contract_address` - The contract address of the ERC-721 token.
    /// * `token_id` - The identifier for an NFT. String representation of the token ID.
    ///
    /// # Returns
    /// * `StdResult<Erc721UriResponse>` - A standard result that wraps the `Erc721UriResponse`.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_uri(
        &self,
        caller: String,
        contract_address: String,
        token_id: String,
    ) -> StdResult<Erc721UriResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Uri {
                caller,
                contract_address,
                token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the hex payload for the ERC-721 `transferFrom` function
    ///
    /// # Arguments
    /// * `from` - Sender Sei native (bech32-encoded 'sei*') address.
    /// * `recipient` - Recipient Sei native (bech32-encoded 'sei*') address.
    /// * `token_id` - The identifier for an NFT. String representation of the token ID.
    ///
    /// # Returns
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`.
    /// `ErcPayloadResponse` contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_transfer_payload(
        &self,
        from: String,
        recipient: String,
        token_id: String,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721TransferPayload {
                from,
                recipient,
                token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the hex payload for the ERC-721 `approve` function
    ///
    /// # Arguments
    /// * `spender` - Spender Sei native (bech32-encoded 'sei*') address.
    /// * `token_id` - The identifier for an NFT. String representation of the token ID.
    ///
    /// # Returns
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`.
    /// `ErcPayloadResponse` contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_approve_payload(
        &self,
        spender: String,
        token_id: String,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721ApprovePayload { spender, token_id },
        }
        .into();

        self.querier.query(&request)
    }

    /// Query to get the hex payload for the ERC-721 `setApprovalForAll` function.
    ///
    /// # Arguments
    /// * `to` - Sei native (bech32-encoded 'sei*') address of the operator
    /// * `approved` - Boolean representing the status to set
    ///
    /// # Returns
    /// * `StdResult<ErcPayloadResponse>` - A standard result that wraps the `ErcPayloadResponse`.
    /// `ErcPayloadResponse` contains the base64-encoded bytes.
    ///
    /// # Errors
    /// This function will return an error if the query fails.
    pub fn erc721_set_approval_all_payload(
        &self,
        to: String,
        approved: bool,
    ) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721SetApprovalAllPayload { to, approved },
        }
        .into();

        self.querier.query(&request)
    }

    /// Queries the EVM (Ethereum Virtual Machine) address associated with a given Sei address.
    ///
    /// This function takes a `sei_address` as a parameter, which is a `String` representing the
    /// SEI address. It returns a `StdResult<EvmAddressResponse>`, which is a standard result type
    /// in the `cosmwasm_std` library. The `EvmAddressResponse` struct contains the EVM address and
    /// a boolean indicating whether the EVM address is associated.
    ///
    /// # Arguments
    ///
    /// * `sei_address` - A `String` that represents the Sei native (bech32-encoded 'sei*') address.
    ///
    /// # Returns
    ///
    /// * `StdResult<EvmAddressResponse>` - A standard result that wraps the `EvmAddressResponse`
    /// struct. `EvmAddressResponse` contains the EVM address and a boolean indicating whether the
    /// EVM address is associated. If the Sei address is not associated with any EVM address,
    /// the EVM address will be an empty string.
    ///
    ///
    /// # Errors
    ///
    /// This function will return an error if the query to the EVM fails.
    pub fn get_evm_address(&self, sei_address: String) -> StdResult<EvmAddressResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::GetEvmAddress { sei_address },
        }
        .into();

        self.querier.query(&request)
    }

    /// Queries the Sei address associated with a given EVM address.
    ///
    /// This function takes an `evm_address` as a parameter, which is a `String` representing the
    /// EVM address. It returns a `StdResult<SeiAddressResponse>`, which is a standard result type
    /// in the `cosmwasm_std` library. The `SeiAddressResponse` struct contains the Sei address.
    /// If the EVM address is not associated with any Sei address, the Sei address will be an empty
    /// string.
    ///
    /// # Arguments
    ///
    /// * `evm_address` - A `String` that represents the EVM address.
    ///
    /// # Returns
    ///
    /// * `StdResult<SeiAddressResponse>` - A standard result that wraps the `SeiAddressResponse`
    /// struct. `SeiAddressResponse` contains the Sei address. If the EVM address is not associated
    /// with any Sei address, the Sei address will be an empty string.
    ///
    /// # Errors
    ///
    /// This function will return an error if the query to the EVM fails.
    pub fn get_sei_address(&self, evm_address: String) -> StdResult<SeiAddressResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::GetSeiAddress { evm_address },
        }
        .into();

        self.querier.query(&request)
    }
}
