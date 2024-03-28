use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const VALUES: Map<u64, u64> = Map::new("values");

pub const USER_SUMS: Map<Addr, u64> = Map::new("user_sums");

pub const PARALLEL_VALS: Map<Addr, u64> = Map::new("parallel_vals");
