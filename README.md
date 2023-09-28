# Sei-Cosmwasm Package

This repository contains the sei-cosmwasm package to support smart contract querying and messages to the modules in sei-chain. It also includes an example contract that can be used to test package behavior locally and can be used as a reference for implementation details to include sei-chain integration in your smart contracts.

## Build Sei Tester Contract

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```

## Testing with LocalSei

### Store Contract Code

`seid tx wasm store artifacts/sei_tester.wasm -y --from <account> --chain-id <name> -b block --gas=3000000 --fees=1000sei`

Make sure to note the code ID for the contract from the tx response. You can also find it in the list of uploaded code with this query:

`seid q wasm list-code`

### Instantiate Contract

`seid tx wasm instantiate <code-id> '{}' -y --no-admin --from <account> --chain-id <name> --gas=1500000 --fees=1000sei -b block --label sei-tester`

Make sure to note the contract address for the contract from the tx response. You can also find it with this query:

`seid q wasm list-contract-by-code <code-id>`

### Query Smart Contract

`seid q wasm contract-state smart <contract-address> <json-query-literal>`

The json literal may look something like this:

`'{"exchange_rates": {}}'`
