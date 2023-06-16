#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, entry_point, to_binary, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdError, StdResult, SubMsg, SubMsgResponse, Uint128,
};

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    types::{OrderData, PositionEffect},
};
use protobuf::Message;
use sei_cosmwasm::{
    BulkOrderPlacementsResponse, ContractOrderResult, DepositInfo, DexTwapsResponse, EpochResponse,
    ExchangeRatesResponse, GetLatestPriceResponse, GetOrderByIdResponse, GetOrdersResponse,
    LiquidationRequest, LiquidationResponse, MsgPlaceOrdersResponse, OracleTwapsResponse, Order,
    OrderSimulationResponse, OrderType, PositionDirection, SeiMsg, SeiQuerier, SeiQueryWrapper,
    SettlementEntry, SudoMsg,
};

const PLACE_ORDER_REPLY_ID: u64 = 1;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sei-tester";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// use semver::Version;

pub fn validate_migration(
    deps: Deps<SeiQueryWrapper>,
    contract_name: &str,
) -> Result<(), StdError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != contract_name {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<SeiMsg>> {
    //validate_migration(_deps.as_ref(), CONTRACT_NAME, CONTRACT_VERSION)?;
    // set the new version
    cw2::set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<SeiQueryWrapper>,
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
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let order_data = OrderData {
        leverage: Decimal::one(),
        position_effect: PositionEffect::Open,
    };

    let order_placement = Order {
        price: Decimal::from_atomics(120u128, 0).unwrap(),
        quantity: Decimal::one(),
        price_denom: "USDC".to_string(),
        asset_denom: "ATOM".to_string(),
        position_direction: PositionDirection::Long,
        order_type: OrderType::Limit,
        data: serde_json::to_string(&order_data).unwrap(),
        status_description: "".to_string(),
        nominal: Decimal::zero(),
    };
    let fund = Coin {
        denom: "uusdc".to_string(),
        amount: Uint128::new(10000000000u128),
    };
    let test_order = sei_cosmwasm::SeiMsg::PlaceOrders {
        funds: vec![fund],
        orders: vec![order_placement],
        contract_address: deps
            .api
            .addr_validate("sei14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sh9m79m")?,
    };
    let test_order_sub_msg = SubMsg::reply_on_success(test_order, PLACE_ORDER_REPLY_ID);
    Ok(Response::new().add_submessage(test_order_sub_msg))
}

pub fn cancel_orders(
    _deps: DepsMut<SeiQueryWrapper>,
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

// create a new coin denom through the tokenfactory module.
// This will create a denom with fullname "factory/{creator address}/{subdenom}"
pub fn create_denom(
    _deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
        subdenom: "subdenom".to_string(),
    };

    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let amount = coin(1000, tokenfactory_denom.to_owned());

    let test_mint = sei_cosmwasm::SeiMsg::MintTokens {
        amount: amount.to_owned(),
    };

    let invalid_amount = coin(1000, tokenfactory_denom);
    let send_msg = SubMsg::new(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![invalid_amount],
    });

    Ok(Response::new()
        .add_message(test_create_denom)
        .add_message(test_mint)
        .add_submessage(send_msg)
    )
}

// mint a token and send to a designated receiver
// note here the denom name provided must be the fullname in format of "factory/{creator address}/{subdenom}"
pub fn mint(
    _deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let amount = coin(100, tokenfactory_denom);

    let test_mint = sei_cosmwasm::SeiMsg::MintTokens {
        amount: amount.to_owned(),
    };
    let send_msg = SubMsg::new(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![amount],
    });

    Ok(Response::new()
        .add_message(test_mint)
        .add_submessage(send_msg))
}

pub fn burn(
    _deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let amount = coin(10, tokenfactory_denom);
    let test_burn = sei_cosmwasm::SeiMsg::BurnTokens { amount };
    Ok(Response::new().add_message(test_burn))
}

pub fn change_admin(
    _deps: DepsMut<SeiQueryWrapper>,
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<SeiMsg>, StdError> {
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
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn handle_new_block(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _epoch: i64,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_order_placements(
    deps: DepsMut<SeiQueryWrapper>,
    _orders: Vec<Order>,
    _deposits: Vec<DepositInfo>,
) -> Result<Response<SeiMsg>, StdError> {
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
    deps.api
        .debug(&format!("process_bulk_order_placements: {:?}", response));
    return Ok(Response::new());
}

pub fn process_bulk_order_cancellations(
    _deps: DepsMut<SeiQueryWrapper>,
    _ids: Vec<u64>,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
}

pub fn process_bulk_liquidation(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _requests: Vec<LiquidationRequest>,
) -> Result<Response<SeiMsg>, StdError> {
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
    deps.api.debug(&format!(
        "pub fn process_bulk_liquidation(
            : {:?}",
        response
    ));
    return Ok(Response::new());
}

pub fn process_finalize_block(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    contract_order_results: Vec<ContractOrderResult>,
) -> Result<Response<SeiMsg>, StdError> {
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
pub fn reply(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    msg: Reply,
) -> Result<Response<SeiMsg>, StdError> {
    match msg.id {
        PLACE_ORDER_REPLY_ID => handle_place_order_reply(deps, msg),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

pub fn handle_place_order_reply(
    deps: DepsMut<SeiQueryWrapper>,
    msg: Reply,
) -> Result<Response<SeiMsg>, StdError> {
    let submsg_response: SubMsgResponse =
        msg.result.into_result().map_err(StdError::generic_err)?;

    match submsg_response.data {
        Some(response_data) => {
            let parsed_order_response: MsgPlaceOrdersResponse =
                Message::parse_from_bytes(response_data.as_slice()).map_err(|_| {
                    StdError::parse_err("MsgPlaceOrdersResponse", "failed to parse data")
                })?;
            deps.api.debug(&format!(
                "Order results from contract {:?}",
                parsed_order_response
            ));

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<SeiQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
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
        QueryMsg::GetLatestPrice {
            contract_address,
            price_denom,
            asset_denom,
        } => to_binary(&query_get_latest_price(
            deps,
            contract_address,
            price_denom,
            asset_denom,
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
    lookback_seconds: u64,
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

pub fn query_get_latest_price(
    deps: Deps<SeiQueryWrapper>,
    contract_address: String,
    price_denom: String,
    asset_denom: String,
) -> StdResult<GetLatestPriceResponse> {
    let valid_addr = deps.api.addr_validate(&contract_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: GetLatestPriceResponse =
        querier.query_get_latest_price(valid_addr, price_denom, asset_denom)?;

    Ok(res)
}
