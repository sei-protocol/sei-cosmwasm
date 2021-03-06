use cosmwasm_std::{Addr, QuerierWrapper, StdResult};

use crate::query::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwapsResponse, SeiQuery, SeiQueryWrapper,
};
use crate::route::SeiRoute;

/// This is a helper wrapper to easily use our custom queries
pub struct SeiQuerier<'a> {
    querier: &'a QuerierWrapper<'a, SeiQueryWrapper>,
}

impl<'a> SeiQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<SeiQueryWrapper>) -> Self {
        SeiQuerier { querier }
    }

    pub fn query_exchange_rates(&self) -> StdResult<ExchangeRatesResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::ExchangeRates {},
        }
        .into();

        self.querier.query(&request)
    }

    pub fn query_oracle_twaps(&self, lookback_seconds: i64) -> StdResult<OracleTwapsResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Oracle,
            query_data: SeiQuery::OracleTwaps { lookback_seconds },
        }
        .into();

        self.querier.query(&request)
    }

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

    pub fn query_epoch(&self) -> StdResult<EpochResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Epoch,
            query_data: SeiQuery::Epoch {},
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
}
