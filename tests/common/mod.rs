use cosmwasm_std::{
    from_binary,
    testing::{MockApi, MockQuerier, MockStorage},
    to_binary, Addr, Api, BalanceResponse, BankMsg, BankQuery, Binary, BlockInfo, CosmosMsg,
    CustomQuery, Empty, MemoryStorage, Querier, Storage, Timestamp,
};
use cw_multi_test::{
    App, AppBuilder, AppResponse, BankKeeper, BankSudo, CosmosRouter, FailingDistribution,
    FailingStaking, Module, Router, SudoMsg, WasmKeeper,
};
use schemars::JsonSchema;
use sei_cosmwasm::{SeiMsg, SeiQuery, SeiQueryWrapper};
use serde::de::DeserializeOwned;

use std::{fmt::Debug, marker::PhantomData};

use anyhow::Result as AnyResult;

pub struct SeiModule(PhantomData<(SeiMsg, SeiQuery)>);

impl SeiModule {
    pub fn new() -> Self {
        SeiModule(PhantomData)
    }
}

impl Default for SeiModule {
    fn default() -> Self {
        Self::new()
    }
}

impl Module for SeiModule {
    type ExecT = SeiMsg;
    type QueryT = SeiQueryWrapper;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        match msg {
            SeiMsg::CreateDenom { subdenom } => {
                let denom = format!("factory/{}/{}", sender, subdenom);
                if storage.get(denom.as_bytes()).is_some() {
                    return Err(anyhow::anyhow!("denom already exists"));
                }

                storage.set(denom.as_bytes(), sender.to_string().as_bytes());

                Ok(AppResponse {
                    events: vec![],
                    data: Some(to_binary(&denom).unwrap()),
                })
            }
            SeiMsg::MintTokens { amount } => {
                let owner = storage.get(amount.denom.as_bytes());
                if owner.is_none() || owner.unwrap() != sender.to_string().as_bytes() {
                    return Err(anyhow::anyhow!(
                        "Must be owner of coin factory denom to mint"
                    ));
                }

                router.sudo(
                    api,
                    storage,
                    block,
                    SudoMsg::Bank(BankSudo::Mint {
                        to_address: sender.to_string(),
                        amount: vec![amount],
                    }),
                )
            }
            SeiMsg::BurnTokens { amount } => {
                let owner = storage.get(amount.denom.as_bytes());
                if owner.is_none() || owner.unwrap() != sender.to_string().as_bytes() {
                    return Err(anyhow::anyhow!(
                        "Must be owner of coin factory denom to burn"
                    ));
                }

                Ok(router
                    .execute(
                        api,
                        storage,
                        block,
                        sender,
                        CosmosMsg::Bank(BankMsg::Burn {
                            amount: vec![amount],
                        }),
                    )
                    .unwrap())
            }
            _ => panic!("Unexpected custom exec msg"),
        }
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: Self::QueryT,
    ) -> AnyResult<Binary> {
        todo!()
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        todo!()
    }
}

pub fn mock_app<F>(
    init_fn: F,
) -> App<
    BankKeeper,
    MockApi,
    MockStorage,
    SeiModule,
    WasmKeeper<SeiMsg, SeiQueryWrapper>,
    FailingStaking,
    FailingDistribution,
>
where
    F: FnOnce(
        &mut Router<
            BankKeeper,
            SeiModule,
            WasmKeeper<SeiMsg, SeiQueryWrapper>,
            FailingStaking,
            FailingDistribution,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    let appbuilder: AppBuilder<
        BankKeeper,
        MockApi,
        MockStorage,
        SeiModule,
        WasmKeeper<SeiMsg, SeiQueryWrapper>,
        FailingStaking,
        FailingDistribution,
    > = AppBuilder::new()
        .with_custom(SeiModule::new())
        .with_wasm::<SeiModule, WasmKeeper<SeiMsg, SeiQueryWrapper>>(WasmKeeper::new());

    appbuilder.build(init_fn)
}
