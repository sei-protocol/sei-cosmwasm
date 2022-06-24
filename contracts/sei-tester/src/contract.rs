use cosmwasm_std::{
    to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, entry_point,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use sei_cosmwasm::{
    ExchangeRatesResponse, SeiMsgWrapper, SeiQuerier, ContractInfoResponse, SeiQueryWrapper,
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
        QueryMsg::ExchangeRates{} => to_binary(&query_exchange_rates(deps)?),
        QueryMsg::ContractInfo{contract_addr} => to_binary(&query_contract_info(deps, contract_addr)?),
    }
}

pub fn query_exchange_rates(
    deps: Deps<SeiQueryWrapper>,
) -> StdResult<ExchangeRatesResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: ExchangeRatesResponse = querier.query_exchange_rates()?;

    Ok(res)
}

pub fn query_contract_info(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
) -> StdResult<ContractInfoResponse> {
    let validated_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: ContractInfoResponse = querier.query_contract_info(validated_addr.clone())?;

    Ok(res)
}
