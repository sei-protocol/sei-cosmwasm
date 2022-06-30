use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use sei_cosmwasm::{ExchangeRatesResponse, OracleTwapsResponse, SeiMsgWrapper, SeiQuerier, SeiQueryWrapper};

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
        QueryMsg::OracleTwaps {lookback_seconds} => to_binary(&query_oracle_twaps(deps, lookback_seconds)?),
    }
}

pub fn query_exchange_rates(deps: Deps<SeiQueryWrapper>) -> StdResult<ExchangeRatesResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates()?;

    Ok(res)
}

pub fn query_oracle_twaps(deps: Deps<SeiQueryWrapper>, lookback_seconds: i64) -> StdResult<OracleTwapsResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: OracleTwapsResponse = querier.query_oracle_twaps(lookback_seconds)?;

    Ok(res)
}
