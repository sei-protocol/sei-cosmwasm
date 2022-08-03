# Sei Bindings for Cosmwasm Contracts

This crate provides Sei specific bindings for cosmwasm contract to be able to interact with the Sei blockchain by exposing custom messages, queries, and structs that correspond to custom module functionality in the Sei chain.

## Installation

Add the sei-cosmwasm dependency to your smart contract's `Cargo.toml` file:

```toml
[dependencies]
sei-cosmwasm = { version = "0.2.0" }
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
            - Creates a denom of `factory/{creator address}/{subdenom}` given the denom creator address and the subdenom. Subdenoms can contain `[a-zA-Z0-9./]`.
        - MintTokens
            - Mint an amount of denom. Minting of a specific denom is only allowed for the creator of the denom registered during `CreateDenom`.
        - BurnTokens
            - Burn an amount of a denom. Burning of a specific denom is only allowed for the creator of the denom registered during `CreateDenom`.
        - ChangeAdmin
            - Burning of a specific denom is only allowed for the creator of the denom registered during `CreateDenom`.

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