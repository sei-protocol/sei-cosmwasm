# Sei Bindings for Cosmwasm Contracts

This crate provides Sei specific bindings for cosmwasm contract to be able to interact with the Sei blockchain by exposing custom messages, queries, and structs that correspond to custom module functionality in the Sei chain.

## Installation

Add the sei-cosmwasm dependency to your smart contract's `Cargo.toml` file:

```toml
[dependencies]
sei-cosmwasm = { version = "0.2.0" }
```

## Functionality

Currently, Sei Bindings support query and message support for the sei custom modules Oracle, Dex, and Epoch. The supported functionality includes the following:

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
    creator: creator_address,
    contract_address: env.contract.address,
    funds: vec![some_funds],
    orders: vec![some_order],
};
Ok(Response::new().add_message(test_order))
```