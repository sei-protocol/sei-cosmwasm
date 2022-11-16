mod msg;
mod proto_structs;
mod querier;
mod query;
mod route;
mod sei_types;
mod tx;

pub use msg::{SeiMsg, SudoMsg};
pub use proto_structs::{
    DenomOracleExchangeRatePair, DexPair, DexTwap, Epoch, OracleExchangeRate, OracleTwap,
};
pub use querier::SeiQuerier;
pub use query::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, GetLatestPriceResponse,
    GetOrderByIdResponse, GetOrdersResponse, OracleTwapsResponse, OrderSimulationResponse,
    PriceResponse, SeiQuery, SeiQueryWrapper,
};
pub use route::SeiRoute;
pub use sei_types::{
    BulkOrderPlacementsResponse, ContractOrderResult, DepositInfo, LiquidationRequest,
    LiquidationResponse, Order, OrderExecutionResult, OrderPlacementResult, OrderResponse,
    OrderStatus, OrderType, PositionDirection, SettlementEntry,
};
pub use tx::MsgPlaceOrdersResponse;

// This export is added to all contracts that import this package, signifying that they require
// "sei" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_sei() {}
