use cosmwasm_std::{QuerierWrapper, StdResult};

use crate::query::{
    ExchangeRatesResponse,
    SeiQuery, SeiQueryWrapper, ContractInfoResponse,
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
        let request = SeiQueryWrapper{
            route: SeiRoute::Oracle,
            query_data: SeiQuery::ExchangeRates {},
        }.into();

        self.querier.query(&request)
    }

    pub fn query_contract_info<T: Into<String>>(
        &self,
        contract_address: T,
    ) -> StdResult<ContractInfoResponse> {
        let request = SeiQueryWrapper {
            route: SeiRoute::Wasm,
            query_data: SeiQuery::ContractInfo {
                contract_address: contract_address.into(),
            },
        }.into();

        self.querier.query(&request)
    }
}
