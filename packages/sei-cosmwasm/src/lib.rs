mod msg;
mod proto_structs;
mod querier;
mod query;
mod route;
mod sei_types;

pub use msg::{SeiMsg, SudoMsg};
pub use proto_structs::{
    DenomOracleExchangeRatePair, DexPair, DexTwap, Epoch, OracleExchangeRate, OracleTwap,
};
pub use querier::SeiQuerier;
pub use query::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, GetOrderByIdResponse,
    GetOrdersResponse, OracleTwapsResponse, OrderSimulationResponse, SeiQuery, SeiQueryWrapper,
};
pub use route::SeiRoute;
pub use sei_types::{
    BulkOrderPlacementsResponse, ContractOrderResult, DepositInfo, LiquidationRequest,
    LiquidationResponse, Order, OrderResponse, OrderType, PositionDirection, SettlementEntry,
};

// This export is added to all contracts that import this package, signifying that they require
// "sei" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_sei() {}
