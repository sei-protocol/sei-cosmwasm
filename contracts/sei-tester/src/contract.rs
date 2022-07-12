use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, Uint64,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use sei_cosmwasm::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, OracleTwapsResponse, SeiMsgWrapper,
    SeiQuerier, SeiQueryWrapper,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<SeiMsgWrapper>> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsgWrapper>, StdError> {
    match msg {}
}

#[entry_point]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::ExchangeRates {} => to_binary(&query_exchange_rates(deps)?),
        QueryMsg::OracleTwaps { lookback_seconds } => {
            to_binary(&query_oracle_twaps(deps, lookback_seconds)?)
        }
        QueryMsg::DexTwaps {
            contract_address,
            lookback_seconds,
        } => to_binary(&query_dex_twaps(deps, contract_address, lookback_seconds)?),
        QueryMsg::Epoch {} => to_binary(&query_epoch(deps)?),
    }
}

pub fn query_exchange_rates(deps: Deps<SeiQueryWrapper>) -> StdResult<ExchangeRatesResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates()?;

    Ok(res)
}

pub fn query_oracle_twaps(
    deps: Deps<SeiQueryWrapper>,
    lookback_seconds: i64,
) -> StdResult<OracleTwapsResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: OracleTwapsResponse = querier.query_oracle_twaps(lookback_seconds)?;

    Ok(res)
}

pub fn query_dex_twaps(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    lookback_seconds: Uint64,
) -> StdResult<DexTwapsResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: DexTwapsResponse = querier.query_dex_twaps(lookback_seconds, valid_addr)?;

    Ok(res)
}

pub fn query_epoch(deps: Deps<SeiQueryWrapper>) -> StdResult<EpochResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: EpochResponse = querier.query_epoch()?;

    Ok(res)
}
