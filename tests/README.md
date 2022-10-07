## Integration Tests

### General Setup

Example tests are provided in `sei_cosmwasm_integration_tests.rs`. The tests use `setup_test()` which utilize `cw_multi_test` to instantiate a wrapper version of a cosmwasm contract, for example, `sei_tester` which can be found in this repo under `contracts/sei_tester`.

A typical integration test will start with:

```rust
let mut app = mock_app(init_default_balances, vec![]);
let sei_tester_addr = setup_test(&mut app);
```

followed by relevant any relevant `Msg` to execute or `Query` to run.

To execute a `MsgToExecute` you can use `execute()` or `execute_multi()`:

```rust
app
.execute_multi(
    addr_to_use,
    vec![CosmosMsg::Custom(SeiMsg::MsgToExecute {
        ...
    })],
)
.unwrap();
```

To query `MsgToQuery` you can use `query_wasm_smart()`:

```rust
app
.wrap()
.query_wasm_smart(
    contract_addr,
    &QueryMsg::MsgToQuery {
        ...
    },
)
```

Module functionality is mocked at the chain level, more details on each module can be found below.

### Dex Module

You can interact with a mocked version of the dex module in the following ways:

Messages:

- `PlaceOrders(orders, funds, contract_address)`: places the corresponding `orders` for the `contract_address`. Each order follows the `Order` struct and has an `order_id`.
- `CancelOrders(order_ids, contract_address)`: cancels the particular `order_ids` for the `contract_address`.

Queries:

- `GetOrders(contract_address, account)`: returns `orders` for a given
- `GetOrderById(contract_address, price_denom, asset_denom, id)`: returns particular `order` based on `id` and price and asset `denom`.

Examples:

- Below is an example where you make an order and call `PlaceOrders()` followed by `GetOrders()`:

First placing an order:

```rust
let mut orders: Vec<Order> = Vec::new();
let mut funds = Vec::<Coin>::new();
let contract_addr = "example_contract".to_string();

// Make order1
let price = Decimal::raw(100);
let quantity = Decimal::raw(1000);
let price_denom = "USDC".to_string();
let asset_denom = "ATOM".to_string();
let order_type = OrderType::Market;
let position_direction = PositionDirection::Long;
let data = "".to_string();
let status_description = "order1".to_string();

let order1: Order = Order {
    price: price,
    quantity: quantity,
    price_denom: price_denom.clone(),
    asset_denom: asset_denom.clone(),
    order_type: order_type,
    position_direction: position_direction,
    data: data,
    status_description: status_description,
};
orders.push(order1);

let res = app
    .execute_multi(
        Addr::unchecked(ADMIN),
        vec![CosmosMsg::Custom(SeiMsg::PlaceOrders {
            orders: orders,
            funds: funds,
            contract_address: Addr::unchecked(&contract_addr),
        })],
    )
    .unwrap();
```

Then querying:

```rust
let res: GetOrdersResponse = app
    .wrap()
    .query_wasm_smart(
        sei_tester_addr.clone(),
        &QueryMsg::GetOrders {
            contract_address: contract_addr.to_string(),
            account: sei_tester_addr.to_string(),
        },
    )
    .unwrap();

assert_eq!(res.orders.len(), 1);
assert_eq!(res.orders[0].id, 0);
assert_eq!(res.orders[0].status, OrderStatus::Placed);
...

```
