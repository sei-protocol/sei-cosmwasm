mod msg;
mod proto_structs;
mod querier;
mod query;
mod route;
mod sei_types;

pub use msg::SeiMsg;
pub use proto_structs::{
    DenomOracleExchangeRatePair, DexPair, DexTwap, Epoch, OracleExchangeRate, OracleTwap,
};
pub use querier::SeiQuerier;
pub use query::{
    CreatorInDenomFeeWhitelistResponse, DexTwapsResponse, EpochResponse, ExchangeRatesResponse,
    GetDenomFeeWhitelistResponse, GetOrderByIdResponse, GetOrdersResponse, OracleTwapsResponse,
    OrderSimulationResponse, SeiQuery, SeiQueryWrapper,
};
pub use route::SeiRoute;
pub use sei_types::{
    ContractOrderResult, MsgPlaceOrdersResponse, Order, OrderExecutionResult, OrderPlacementResult,
    OrderResponse, OrderType, PositionDirection,
};

// This export is added to all contracts that import this package, signifying that they require
// "sei" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_sei() {}
