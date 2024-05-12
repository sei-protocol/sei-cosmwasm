# Sei-Cosmwasm Package

## Introduction

The sei-cosmwasm package is a comprehensive toolkit designed for developers looking to integrate sei-chain functionalities into their Cosmos smart contracts. Leveraging the power of CosmWasm, this package simplifies the process of querying blockchain state and sending messages to sei-chain modules directly from your smart contracts. Whether you're building complex DeFi platforms or simple token contracts, sei-cosmwasm offers the building blocks necessary for seamless integration.

## Prerequisites

Before you begin, ensure you have the following installed and configured:

- Docker: For building and deploying contracts.
- Rust: Latest stable version.
- Sei-chain CLI (seid): For interacting with the sei-chain network.

A basic understanding of smart contract development and the Cosmos SDK will also be beneficial.

## Build Sei Tester Contract

To build the sei tester contract, run the following command:

```shell
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
```

This command uses the `cosmwasm/rust-optimizer:0.14.0` Docker image to compile your smart contracts, ensuring a consistent and optimized build environment.

## Testing with LocalSei

### Store Contract Code

Store your wasm contract on the blockchain:

```shell
seid tx wasm store artifacts/sei_tester.wasm -y --from <account> --chain-id <name> -b block --gas=3000000 --fees=1000sei
```

- `<account>`: Your sei-chain account name.
- `<name>`: The chain ID of your local sei-chain instance.

After storing, note the code ID from the transaction response.

### Instantiate Contract

Deploy your contract with:

```shell
seid tx wasm instantiate <code-id> '{}' -y --no-admin --from <account> --chain-id <name> --gas=1500000 --fees=1000sei -b block --label sei-tester
```

Replace `<code-id>` with the ID obtained in the previous step.

### Query Smart Contract

Interact with your contract:

```shell
seid q wasm contract-state smart <contract-address> '{"exchange_rates": {}}'
```

- `<contract-address>`: The address of your instantiated contract.

This query fetches exchange rates, as an example. Adjust the JSON query literal based on the data you wish to query.

## Additional Resources

For more detailed technical documentation, visit our [Developer Guides](https://docs.sei.io/develop/get-started). Join our community on [Discord](https://discord.gg/sei) for support and discussions.

## Changelog

For a detailed list of changes and updates, refer to our [CHANGELOG.md](#).

## Contributing

We welcome contributions from the community! If you're interested in improving the sei-cosmwasm package, please review our [contribution guidelines](#) for information on submitting pull requests and reporting bugs.
