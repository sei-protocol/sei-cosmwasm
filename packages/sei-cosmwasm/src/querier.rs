use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{ExchangeRatesResponse, OracleTwapsResponse, SeiQuery, SeiQueryWrapper};
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
            query_data: SeiQuery::OracleTwaps {lookback_seconds: lookback_seconds},
        }
        .into();

        self.querier.query(&request)
    }
}
