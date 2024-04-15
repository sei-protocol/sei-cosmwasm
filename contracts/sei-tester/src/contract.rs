use cosmwasm_std::to_json_binary;
// #[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, entry_point, Attribute, BankMsg, Binary, Coin, Decimal, Deps, DepsMut, Env, MessageInfo,
    Order as IteratorOrder, Reply, Response, StdError, StdResult, SubMsg, SubMsgResponse, Uint128,
};
use cw_storage_plus::Bound;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{PARALLEL_VALS, USER_SUMS, VALUES},
    types::{OrderData, PositionEffect},
};
use protobuf::Message;
use sei_cosmwasm::{
    BulkOrderPlacementsResponse, Cancellation, DenomAuthorityMetadataResponse, DenomUnit,
    DenomsFromCreatorResponse, DepositInfo, DexTwapsResponse, EpochResponse, EvmAddressResponse,
    ExchangeRatesResponse, GetLatestPriceResponse, GetOrderByIdResponse, GetOrdersResponse,
    Metadata, MsgPlaceOrdersResponse, OracleTwapsResponse, Order, OrderSimulationResponse,
    OrderType, PositionDirection, SeiAddressResponse, SeiMsg, SeiQuerier, SeiQueryWrapper,
    SettlementEntry, SudoMsg, StaticCallResponse,
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
        return Err(StdError::generic_err("Can only upgrade from same type"));
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
        ExecuteMsg::SetMetadata {} => set_metadata(deps, env, info),
        ExecuteMsg::TestOccIteratorWrite { values } => {
            test_occ_iterator_write(deps, env, info, values)
        }
        ExecuteMsg::TestOccIteratorRange { start, end } => {
            test_occ_iterator_range(deps, env, info, start, end)
        }
        ExecuteMsg::TestOccParallelism { value } => test_occ_parallelism(deps, env, info, value),
        ExecuteMsg::CallEvm { value, to, data } => {
            let test_call_evm = SeiMsg::CallEvm { value, to, data };
            Ok(Response::new().add_message(test_call_evm))
        }
    }
}

fn test_occ_iterator_write(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
    values: Vec<(u64, u64)>,
) -> Result<Response<SeiMsg>, StdError> {
    // writes all of the values (index, value) to the store
    for (key, value) in values {
        VALUES.save(deps.storage, key, &value)?;
    }
    Ok(Response::new())
}

fn test_occ_iterator_range(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    start: u64,
    end: u64,
) -> Result<Response<SeiMsg>, StdError> {
    // iterates through the `VALUES` and for all that exist, sums them and writes them to user_sums for the sender
    let mut sum: u64 = 0;
    let values: Vec<(u64, u64)> = VALUES
        .range(
            deps.storage,
            Some(Bound::inclusive(start)),
            Some(Bound::inclusive(end)),
            IteratorOrder::Ascending,
        )
        .collect::<Result<Vec<(u64, u64)>, StdError>>()
        .unwrap();

    let mut value_attrs: Vec<Attribute> = vec![];
    for (key, val) in values {
        sum += val;
        value_attrs.push(Attribute::new(key.to_string(), val.to_string()));
    }
    USER_SUMS.save(deps.storage, info.sender.clone(), &sum)?;

    Ok(Response::new()
        .add_attribute("user", info.sender.to_string())
        .add_attribute("sum", sum.to_string())
        .add_attributes(value_attrs))
}

fn test_occ_parallelism(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    value: u64,
) -> Result<Response<SeiMsg>, StdError> {
    // writes the value to the store for the sender
    PARALLEL_VALS.save(deps.storage, info.sender.clone(), &value)?;
    Ok(Response::new()
        .add_attribute("user", info.sender.to_string())
        .add_attribute("val", value.to_string()))
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
    let mut cancellations: Vec<Cancellation> = vec![];
    for id in order_ids {
        cancellations.push(Cancellation {
            id,
            contract_address: env.contract.address.to_string(),
            price_denom: "USDC".to_string(),
            asset_denom: "ATOM".to_string(),
            price: Decimal::from_atomics(120u128, 0).unwrap(),
            position_direction: PositionDirection::Long,
            order_type: OrderType::Limit,
        });
    }
    let test_cancel = sei_cosmwasm::SeiMsg::CancelOrders {
        cancellations,
        contract_address: env.contract.address,
    };
    Ok(Response::new().add_message(test_cancel))
}

// create a new coin denom through the tokenfactory module.
// This will create a denom with fullname "factory/{creator address}/{subdenom}"
pub fn create_denom(
    _deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
        subdenom: "subdenom".to_string(),
    };
    Ok(Response::new().add_message(test_create_denom))
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

// set coin metadata for a tokenfactory denom.
pub fn set_metadata(
    _deps: DepsMut<SeiQueryWrapper>,
    env: Env,
    _info: MessageInfo,
) -> Result<Response<SeiMsg>, StdError> {
    let tokenfactory_denom =
        "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
    let test_metadata = Metadata {
        description: "Token Metadata".to_string(),
        base: tokenfactory_denom.clone(),
        display: "SUBDENOM".to_string(),
        name: "subdenom".to_string(),
        symbol: "SUB".to_string(),
        denom_units: vec![
            DenomUnit {
                denom: tokenfactory_denom,
                exponent: 0,
                aliases: vec!["usubdenom".to_string()],
            },
            DenomUnit {
                denom: "SUBDENOM".to_string(),
                exponent: 6,
                aliases: vec!["subdenom".to_string()],
            },
        ],
    };
    let test_set_metadata = sei_cosmwasm::SeiMsg::SetMetadata {
        metadata: test_metadata,
    };
    Ok(Response::new().add_message(test_set_metadata))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut<SeiQueryWrapper>,
    _env: Env,
    msg: SudoMsg,
) -> Result<Response<SeiMsg>, StdError> {
    match msg {
        SudoMsg::Settlement { epoch, entries } => process_settlements(deps, entries, epoch),
        SudoMsg::BulkOrderPlacements { orders, deposits } => {
            process_bulk_order_placements(deps, orders, deposits)
        }
        SudoMsg::BulkOrderCancellations { ids } => process_bulk_order_cancellations(deps, ids),
    }
}

pub fn process_settlements(
    _deps: DepsMut<SeiQueryWrapper>,
    _entries: Vec<SettlementEntry>,
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
    Ok(Response::new())
}

pub fn process_bulk_order_cancellations(
    _deps: DepsMut<SeiQueryWrapper>,
    _ids: Vec<u64>,
) -> Result<Response<SeiMsg>, StdError> {
    Ok(Response::new())
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
        QueryMsg::ExchangeRates {} => to_json_binary(&query_exchange_rates(deps)?),
        QueryMsg::OracleTwaps { lookback_seconds } => {
            to_json_binary(&query_oracle_twaps(deps, lookback_seconds)?)
        }
        QueryMsg::DexTwaps {
            contract_address,
            lookback_seconds,
        } => to_json_binary(&query_dex_twaps(deps, contract_address, lookback_seconds)?),
        QueryMsg::OrderSimulation {
            order,
            contract_address,
        } => to_json_binary(&query_order_simulation(deps, order, contract_address)?),
        QueryMsg::Epoch {} => to_json_binary(&query_epoch(deps)?),
        QueryMsg::GetOrders {
            contract_address,
            account,
        } => to_json_binary(&query_get_orders(deps, contract_address, account)?),
        QueryMsg::GetOrderById {
            contract_address,
            price_denom,
            asset_denom,
            id,
        } => to_json_binary(&query_get_order_by_id(
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
        } => to_json_binary(&query_get_latest_price(
            deps,
            contract_address,
            price_denom,
            asset_denom,
        )?),
        QueryMsg::GetDenomAuthorityMetadata { denom } => {
            to_json_binary(&query_denom_authority_metadata(deps, denom)?)
        }
        QueryMsg::GetDenomsFromCreator { creator } => {
            to_json_binary(&query_denoms_from_creator(deps, creator)?)
        }
        QueryMsg::StaticCall { from, to, data } => {
            to_json_binary(&query_static_call(deps, from, to, data)?)
        }
        QueryMsg::GetEvmAddressBySeiAddress { sei_address } => {
            to_json_binary(&query_evm_address(deps, sei_address)?)
        }
        QueryMsg::GetSeiAddressByEvmAddress { evm_address } => {
            to_json_binary(&query_sei_address(deps, evm_address)?)
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

pub fn query_denom_authority_metadata(
    deps: Deps<SeiQueryWrapper>,
    denom: String,
) -> StdResult<DenomAuthorityMetadataResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res: DenomAuthorityMetadataResponse = querier.query_denom_authority_metadata(denom)?;

    Ok(res)
}

pub fn query_denoms_from_creator(
    deps: Deps<SeiQueryWrapper>,
    creator: String,
) -> StdResult<DenomsFromCreatorResponse> {
    let creator_addr = deps.api.addr_validate(&creator)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: DenomsFromCreatorResponse = querier.query_denoms_from_creator(creator_addr)?;

    Ok(res)
}

pub fn query_static_call(
    deps: Deps<SeiQueryWrapper>,
    from: String,
    to: String,
    data: String,
) -> StdResult<StaticCallResponse> {
    let valid_from_addr = deps.api.addr_validate(&from)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res: StaticCallResponse = querier.static_call(valid_from_addr.to_string(), to, data)?;

    Ok(res)
}

pub fn query_evm_address(
    deps: Deps<SeiQueryWrapper>,
    sei_address: String,
) -> StdResult<EvmAddressResponse> {
    let valid_addr = deps.api.addr_validate(&sei_address)?;
    let querier = SeiQuerier::new(&deps.querier);
    let res = querier.get_evm_address(valid_addr.to_string())?;

    Ok(res)
}

pub fn query_sei_address(
    deps: Deps<SeiQueryWrapper>,
    evm_address: String,
) -> StdResult<SeiAddressResponse> {
    let querier = SeiQuerier::new(&deps.querier);
    let res = querier.get_sei_address(evm_address)?;

    Ok(res)
}
