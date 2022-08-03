use cosmwasm_std::{
    coin, entry_point, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, QueryResponse,
    Response, StdError, StdResult,
};

use crate::msg::{
    BulkOrderPlacementsResponse, DepositInfo, ExecuteMsg, InstantiateMsg, LiquidationRequest,
    LiquidationResponse, QueryMsg, SettlementEntry, SudoMsg,
};
use sei_cosmwasm::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwapsResponse, Order, OrderType, PositionDirection, SeiMsg,
    SeiQuerier, SeiQueryWrapper,
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
        ExecuteMsg::CancelOrders { order_ids } => cancel_orders(deps, env, info, order_ids),
        ExecuteMsg::CreateDenom {} => create_denom(deps, env, info),
        ExecuteMsg::Mint {} => mint(deps, env, info),
        ExecuteMsg::Burn {} => burn(deps, env, info),
        ExecuteMsg::ChangeAdmin {} => change_admin(deps, env, info),
    }
}

pub fn place_orders(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let order_placement = Order {
        price: Decimal::from_atomics(120u128, 0).unwrap(),
        quantity: Decimal::one(),
        price_denom: "sei".to_string(),
        asset_denom: "atom".to_string(),
        position_direction: PositionDirection::Long,
        order_type: OrderType::Limit,
        data: "".to_string(),
        status_description: "".to_string(),
    };
    let test_order = sei_cosmwasm::SeiMsg::PlaceOrders {
        creator: env.contract.address.clone(),
        contract_address: env.contract.address,
        funds: vec![],
        orders: vec![order_placement],
    };
    Ok(Response::new().add_message(test_order))
}

pub fn cancel_orders(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    order_ids: Vec<u64>,
) -> Result<Response<SeiMsg>, StdError> {
    let test_cancel = sei_cosmwasm::SeiMsg::CancelOrders {
        creator: env.contract.address.clone(),
        contract_address: env.contract.address,
        order_ids,
    };
    Ok(Response::new().add_message(test_cancel))
}

pub fn create_denom(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
        subdenom: "subdenom".to_string(),
    };
    Ok(Response::new().add_message(test_create_denom))
}

pub fn mint(_deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let amount = coin(100, tokenfactory_denom);
    let test_mint = sei_cosmwasm::SeiMsg::MintTokens { amount };
    Ok(Response::new().add_message(test_mint))
}

pub fn burn(_deps: DepsMut, env: Env, _info: MessageInfo) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let amount = coin(10, tokenfactory_denom);
    let test_burn = sei_cosmwasm::SeiMsg::BurnTokens { amount };
    Ok(Response::new().add_message(test_burn))
}

pub fn change_admin(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let new_admin_address = "sei1hjfwcza3e3uzeznf3qthhakdr9juetl7g6esl4".to_string();
    let test_change_admin = sei_cosmwasm::SeiMsg::ChangeAdmin {
        denom: tokenfactory_denom,
        new_admin_address,
    };
    Ok(Response::new().add_message(test_change_admin))
}

#[entry_point]
pub fn sudo(deps: DepsMut<SeiQueryWrapper>, env: Env, msg: SudoMsg) -> Result<Response, StdError> {
    match msg {
        SudoMsg::Settlement { epoch, entries } => process_settlements(deps, entries, epoch),
        SudoMsg::NewBlock { epoch } => handle_new_block(deps, env, epoch),
        SudoMsg::BulkOrderPlacements { orders, deposits } => {
            process_bulk_order_placements(deps, orders, deposits)
        }
        SudoMsg::BulkOrderCancellations { ids } => process_bulk_order_cancellations(deps, ids),
        SudoMsg::Liquidation { requests } => process_bulk_liquidation(deps, env, requests),
    }
}

pub fn process_settlements(
    _deps: DepsMut<SeiQueryWrapper>,
    _entries: Vec<SettlementEntry>,
    _epoch: i64,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

pub fn handle_new_block(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _epoch: i64,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_order_placements(
    _deps: DepsMut<SeiQueryWrapper>,
    _orders: Vec<Order>,
    _deposits: Vec<DepositInfo>,
) -> Result<Response, StdError> {
    let response = BulkOrderPlacementsResponse {
        unsuccessful_orders: vec![],
    };
    let serialized_json = match serde_json::to_string(&response) {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing response: {:?}", error),
    };
    let base64_json_str = base64::encode(serialized_json);
    let binary = match Binary::from_base64(base64_json_str.as_ref()) {
        Ok(val) => val,
        Err(error) => panic!("Problem converting binary for order request: {:?}", error),
    };

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    Ok(response)
}

pub fn process_bulk_order_cancellations(
    _deps: DepsMut<SeiQueryWrapper>,
    _ids: Vec<u64>,
) -> Result<Response, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_liquidation(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _requests: Vec<LiquidationRequest>,
) -> Result<Response, StdError> {
    let response = LiquidationResponse {
        successful_accounts: vec![],
        liquidation_orders: vec![],
    };
    let serialized_json = match serde_json::to_string(&response) {
        Ok(val) => val,
        Err(error) => panic!("Problem parsing response: {:?}", error),
    };
    let base64_json_str = base64::encode(serialized_json);
    let binary = match Binary::from_base64(base64_json_str.as_ref()) {
        Ok(val) => val,
        Err(error) => panic!("Problem converting binary for order request: {:?}", error),
    };

    let mut response: Response = Response::new();
    response = response.set_data(binary);
    Ok(response)
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
        QueryMsg::GetOrders {
            contract_address,
            account,
        } => to_binary(&query_get_orders(deps, contract_address, account)?),
        QueryMsg::GetOrderById {
            contract_address,
            price_denom,
            asset_denom,
            id,
        } => to_binary(&query_get_order_by_id(
            deps,
            contract_address,
            price_denom,
            asset_denom,
            id,
        )?),
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

pub fn query_get_orders(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    account: String,
) -> StdResult<GetOrdersResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let valid_acc = deps.api.addr_validate(&account)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetOrdersResponse = querier.query_get_orders(valid_addr, valid_acc)?;

    Ok(res)
}

pub fn query_get_order_by_id(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    price_denom: String,
    asset_denom: String,
    order_id: u64,
) -> StdResult<GetOrderByIdResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetOrderByIdResponse =
        querier.query_get_order_by_id(valid_addr, price_denom, asset_denom, order_id)?;

    Ok(res)
}
