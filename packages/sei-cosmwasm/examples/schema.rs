use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use sei_cosmwasm::{
    DexTwapsResponse, ExchangeRatesResponse, OracleTwapsResponse, SeiMsg, SeiQuery,
    SeiQueryWrapper, SeiRoute,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(SeiMsg), &out_dir);
    export_schema(&schema_for!(SeiQueryWrapper), &out_dir);
    export_schema(&schema_for!(SeiQuery), &out_dir);
    export_schema(&schema_for!(SeiRoute), &out_dir);
    export_schema(&schema_for!(ExchangeRatesResponse), &out_dir);
    export_schema(&schema_for!(OracleTwapsResponse), &out_dir);
    export_schema(&schema_for!(DexTwapsResponse), &out_dir);
}
