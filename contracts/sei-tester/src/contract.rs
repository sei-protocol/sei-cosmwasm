use cosmwasm_std::{
    coin, entry_point, from_binary, to_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, StdError, StdResult, SubMsg, SubMsgResponse,
};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    types::{OrderData, PositionEffect},
};
use sei_cosmwasm::{
    BulkOrderPlacementsResponse, ContractOrderResult, CreatorInDenomFeeWhitelistResponse,
    DepositInfo, DexTwapsResponse, EpochResponse, ExchangeRatesResponse,
    GetDenomFeeWhitelistResponse, GetOrderByIdResponse, GetOrdersResponse, LiquidationRequest,
    LiquidationResponse, MsgPlaceOrdersResponse, OracleTwapsResponse, Order,
    OrderSimulationResponse, OrderType, PositionDirection, SeiMsg, SeiQuerier, SeiQueryWrapper,
    SettlementEntry, SudoMsg,
};

const PLACE_ORDER_REPLY_ID: u64 = 1;

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
    let order_data = OrderData {
        leverage: Decimal::one(),
        position_effect: PositionEffect::Open,
    };

    let order_placement = Order {
        price: Decimal::from_atomics(120u128, 0).unwrap(),
        quantity: Decimal::one(),
        price_denom: "sei".to_string(),
        asset_denom: "atom".to_string(),
        position_direction: PositionDirection::Long,
        order_type: OrderType::Limit,
        data: serde_json::to_string(&order_data).unwrap(),
        status_description: "".to_string(),
    };
    let test_order = sei_cosmwasm::SeiMsg::PlaceOrders {
        funds: vec![],
        orders: vec![order_placement],
        contract_address: env.contract.address,
    };
    let test_order_sub_msg = SubMsg::reply_on_success(test_order, PLACE_ORDER_REPLY_ID);
    Ok(Response::new().add_submessage(test_order_sub_msg))
}

pub fn cancel_orders(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    order_ids: Vec<u64>,
) -> Result<Response<SeiMsg>, StdError> {
    let test_cancel = sei_cosmwasm::SeiMsg::CancelOrders {
        order_ids,
        contract_address: env.contract.address,
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
        SudoMsg::FinalizeBlock {
            contract_order_results,
        } => process_finalize_block(deps, env, contract_order_results),
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

pub fn process_finalize_block(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    contract_order_results: Vec<ContractOrderResult>,
) -> Result<Response, StdError> {
    deps.api.debug("Processing finalize block...");

    // print order placement results
    for order_results in contract_order_results {
        deps.api.debug(&format!(
            "Order results from contract {}",
            order_results.contract_address
        ));

        for order_placement in order_results.order_placement_results {
            deps.api.debug(&format!(
                "Order id {}, status {}",
                order_placement.order_id, order_placement.status_code
            ));
        }
        for order_execution in order_results.order_execution_results {
            deps.api.debug(&format!(
                "Order id {}, executed_quantity {}",
                order_execution.order_id, order_execution.executed_quantity
            ));
        }
    }

    let response = Response::new();
    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut<SeiQueryWrapper>, _env: Env, msg: Reply) -> Result<Response, StdError> {
    match msg.id {
        PLACE_ORDER_REPLY_ID => handle_place_order_reply(msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

pub fn handle_place_order_reply(msg: Reply) -> Result<Response, StdError> {
    let submsg_response: SubMsgResponse =
        msg.result.into_result().map_err(StdError::generic_err)?;

    match submsg_response.data {
        Some(response_data) => {
            let parsed_order_response: MsgPlaceOrdersResponse = from_binary(&response_data)?;
            Ok(Response::new()
                .add_attribute("method", "handle_place_order_reply")
                .add_attribute(
                    "order_ids",
                    format!("{:?}", parsed_order_response.order_ids),
                ))
        }
        None => Ok(Response::new().add_attribute("method", "handle_place_order_reply")),
    }
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
        QueryMsg::OrderSimulation {
            order,
            contract_address,
        } => to_binary(&query_order_simulation(deps, order, contract_address)?),
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
        QueryMsg::GetDenomFeeWhitelist {} => to_binary(&query_get_denom_fee_whitelist(deps)?),
        QueryMsg::CreatorInDenomFeeWhitelist { creator } => {
            to_binary(&query_creator_in_denom_fee_whitelist(deps, creator)?)
        }
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

pub fn query_order_simulation(
    deps: Deps<SeiQueryWrapper>,
    order: Order,
    contract_address: String,
) -> StdResult<OrderSimulationResponse> {
    let contract_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: OrderSimulationResponse = querier.query_order_simulation(order, contract_addr)?;

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

pub fn query_get_denom_fee_whitelist(
    deps: Deps<SeiQueryWrapper>,
) -> StdResult<GetDenomFeeWhitelistResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetDenomFeeWhitelistResponse = querier.query_get_denom_fee_whitelist()?;

    Ok(res)
}

pub fn query_creator_in_denom_fee_whitelist(
    deps: Deps<SeiQueryWrapper>,
    creator: String,
) -> StdResult<CreatorInDenomFeeWhitelistResponse> {
    let valid_addr = deps.api.addr_validate(&creator)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: CreatorInDenomFeeWhitelistResponse =
        querier.query_creator_in_denom_fee_whitelist(valid_addr)?;

    Ok(res)
}
