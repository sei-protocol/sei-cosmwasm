mod msg;
mod proto_structs;
mod querier;
mod query;
mod route;

pub use msg::{SeiMsg, SeiMsgWrapper};
pub use proto_structs::{
    DenomOracleExchangeRatePair, DexPair, DexTwap, Epoch, OracleExchangeRate, OracleTwap,
};
pub use querier::SeiQuerier;
pub use query::{
    DexTwapsResponse, EpochResponse, ExchangeRatesResponse, OracleTwapsResponse, SeiQuery,
    SeiQueryWrapper,
};
pub use route::SeiRoute;

// TODO: properly support this requirement behavior in sei-chain
// // This export is added to all contracts that import this package, signifying that they require
// // "sei" support on the chain they run on.
// #[no_mangle]
// extern "C" fn requires_sei() {}
