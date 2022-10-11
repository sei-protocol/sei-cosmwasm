# Sei Bindings for Cosmwasm Contracts

This crate provides Sei specific bindings for cosmwasm contract to be able to interact with the Sei blockchain by exposing custom messages, queries, and structs that correspond to custom module functionality in the Sei chain.

## Installation

Add the sei-cosmwasm dependency to your smart contract's `Cargo.toml` file:

```toml
[dependencies]
sei-cosmwasm = { version = "0.4.6" }
```

## Functionality

Currently, Sei Bindings support query and message support for the sei custom modules Oracle, Dex, Epoch and TokenFactory. The supported functionality includes the following:

- Oracle
    - Query
        - ExchangeRates
            - Gets the exchange rates for supported assets
        - OracleTwaps
            - Gets the time weighted average price for supported assets
- Dex
    - Query
        - DexTwaps
            - Gets time weighted average prices for assets on the specific order book
        - GetOrders
            - Get orders by a specific account on the dex order book
        - GetOrderById
            - Get individual order by order ID
    - Message
        - PlaceOrders
            - Bulk place orders with the dex order book
        - CancelOrders
            - Bulk cancel orders with the dex order book
- Epoch
    - Query
        - Epoch
            - Get current epoch information
- TokenFactory
    - Message
        - CreateDenom
            - Creates a denom of type `factory/{creator address}/{subdenom}` given a `subdenom`.
        - MintTokens
            - Mint an amount of a factory denom. Only the creator of the denom (admin) can mint.
        - BurnTokens
            - Burns an amount of a factory denom. Only the creater of the denom (admin) can mint.
        - ChangeAdmin
            - Change the Admin of the Denom. Only the current admin can change the admin.

## Usage

### Querying

To use these custom queries with the sei chain, you can create an instance of `SeiQuerier` and call the query helper functions with the appropriate parameters.

```rust
let querier = SeiQuerier::new(&deps.querier);
let res: ExchangeRatesResponse = querier.query_exchange_rates()?;
```

### Messages

To use the custom messages, the messages need to be returned in the contract response to be executed by the sei chain.

```rust
let test_order = sei_cosmwasm::SeiMsg::PlaceOrders {
    contract_address: env.contract.address,
    funds: vec![some_funds],
    orders: vec![some_order],
};
Ok(Response::new().add_message(test_order))
```

### Tokenfactory

The tokenfactory supports any Sei user to create, mint, burn and change owner of custom tokens. 


```rust
// create a new coin denom through the tokenfactory module.
// This will create a denom with fullname "factory/{creator address}/{subdenom}"
let test_create_denom = sei_cosmwasm::SeiMsg::CreateDenom {
    subdenom: "subdenom".to_string(),
};
Ok(Response::new().add_message(test_create_denom))


// mint a token and send to a designated receiver
// note here the denom name provided must be the fullname in format of "factory/{creator address}/{subdenom}"
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


// burn a token, the denom name provided must be the fullname in format of "factory/{creator address}/{subdenom}"
let tokenfactory_denom =
    "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
let amount = coin(10, tokenfactory_denom);
let test_burn = sei_cosmwasm::SeiMsg::BurnTokens { amount };
Ok(Response::new().add_message(test_burn))

// change the owner of a token 
let tokenfactory_denom =
    "factory/".to_string() + env.contract.address.to_string().as_ref() + "/subdenom";
let new_admin_address = "${NEW_ADMIN_ADDRESS}".to_string();
let test_change_admin = sei_cosmwasm::SeiMsg::ChangeAdmin {
    denom: tokenfactory_denom,
    new_admin_address,
};
Ok(Response::new().add_message(test_change_admin))
```
