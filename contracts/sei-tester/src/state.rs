use cosmwasm_std::{Addr, Decimal, Uint128, Uint64};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use sei_cosmwasm::ExchangeRatesResponse;
use serde::{Deserialize, Serialize};


pub const VALUES: Map<u64, u64> =
    Map::new("values");

pub const USER_SUMS: Map<Addr, u64> =
    Map::new("user_sums");

pub const PARALLEL_VALS: Map<Addr, u64> =
    Map::new("parallel_vals");