use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
    StdResult, Decimal,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use sei_cosmwasm::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, OracleTwapsResponse,
    SeiQuerier, SeiQueryWrapper, SeiMsg, OrderPlacement, OrderType, PositionDirection, PositionEffect,
};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<SeiMsg>> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<SeiMsg>, StdError> {
    match msg {
        ExecuteMsg::PlaceOrders {} => place_orders(deps, env, info),
        ExecuteMsg::CancelOrders {} => cancel_orders(deps, env, info),
    }
}

pub fn place_orders(_deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response<SeiMsg>, StdError> {
    let order_placement = OrderPlacement{
        position_direction: PositionDirection::Long,
        price: Decimal::from_atomics(120u128, 0).unwrap(),
        quantity: Decimal::one(),
        price_denom: "usei".to_string(),
        asset_denom: "uatom".to_string(),
        position_effect: PositionEffect::Open,
        order_type: OrderType::Limit,
        leverage: Decimal::one(),
    };

    let test_order = sei_cosmwasm::SeiMsg::PlaceOrders{
        creator: env.contract.address.clone(),
        contract_address: env.contract.address.clone(),
        funds: vec![],
        orders: vec![order_placement],
    };
    Ok(Response::new().add_message(test_order))
}

pub fn cancel_orders(_deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response<SeiMsg>, StdError> {
    let test_cancel = sei_cosmwasm::SeiMsg::CancelOrders{
        creator: env.contract.address.clone(),
        contract_address: env.contract.address.clone(),
        order_ids: vec![],
    };
    Ok(Response::new().add_message(test_cancel))
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
    lookback_seconds: u64,
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
