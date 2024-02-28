use cosmwasm_std::{Addr, QuerierWrapper, StdResult, Uint128};
use cw20::{TokenInfoResponse, BalanceResponse};

use crate::query::{
    DenomAuthorityMetadataResponse, DenomsFromCreatorResponse, DexTwapsResponse, EpochResponse,
    ExchangeRatesResponse, GetLatestPriceResponse, GetOrderByIdResponse, GetOrdersResponse,
    OracleTwapsResponse, OrderSimulationResponse, StaticCallResponse, ErcPayloadResponse,
    Erc20AllowanceResponse, Erc721OwnerResponse, Erc721ApprovedResponse, Erc721IsApprovedForAllResponse,
    Erc721NameSymbolResponse, Erc721UriResponse, SeiQuery, SeiQueryWrapper,
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

    pub fn static_call(&self, from: String, to: String, data: String) -> StdResult<StaticCallResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::StaticCall {
                from, to, data,
            },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc20_transfer_payload(&self, recipient: String, amount: Uint128) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TransferPayload {
                recipient, amount,
            },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc20_transfer_from_payload(&self, owner: String, recipient: String, amount: Uint128) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TransferFromPayload {
                owner, recipient, amount,
            },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc20_approve_payload(&self, spender: String, amount: Uint128) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20ApprovePayload {
                spender, amount,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc20_allowance(&self, contract_address: String, owner: String, spender: String) -> StdResult<Erc20AllowanceResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20Allowance {
                contract_address, owner, spender,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc20_token_info(&self, contract_address: String, caller: String) -> StdResult<TokenInfoResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20TokenInfo {
                contract_address, caller,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc20_balance(&self, contract_address: String, account: String) -> StdResult<BalanceResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc20Balance {
                contract_address, account,
            },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc721_owner(&self, caller: String, contract_address: String, token_id: String) -> StdResult<Erc721OwnerResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Owner { caller, contract_address, token_id },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc721_approved(&self, caller: String, contract_address: String, token_id: String) -> StdResult<Erc721ApprovedResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Approved { caller, contract_address, token_id },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc721_is_approved_for_all(&self, caller: String, contract_address: String, owner: String, operator: String) -> StdResult<Erc721IsApprovedForAllResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721IsApprovedForAll { caller, contract_address, owner, operator },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc721_name_symbol(&self, caller: String, contract_address: String) -> StdResult<Erc721NameSymbolResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721NameSymbol { caller, contract_address },
        }
        .into();

        self.querier.query(&request)
    }

    pub fn erc721_uri(&self, caller: String, contract_address: String, token_id: String,) -> StdResult<Erc721UriResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721Uri { caller, contract_address, token_id },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc721_transfer_payload(&self, from: String, recipient: String, token_id: String) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721TransferPayload {
                from, recipient, token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc721_approve_payload(&self, spender: String, token_id: String) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721ApprovePayload {
                spender, token_id,
            },
        }
        .into();

        self.querier.query(&request)
    }

    // returns base64-encoded bytes
    pub fn erc721_set_approval_all_payload(&self, to: String, approved: bool) -> StdResult<ErcPayloadResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Evm,
            query_data: SeiQuery::Erc721SetApprovalAllPayload { to, approved, },
        }
        .into();

        self.querier.query(&request)
    }
}
