# Osmosis Swaprouter Worskhop

This is a workshop for building an Osmosis Swap Router CosmWasm contract.

The original contract repository is located here:
https://github.com/osmosis-labs/swaprouter

Original authors:
- [sunnya97](https://github.com/sunnya97)
- [nicolaslara](https://github.com/nicolaslara)
- [iboss-ptk](https://github.com/iboss-ptk)

## What Is This

A contract that allows to swap an **exact** amount of tokens for a **minimum** of another token, similar to swapping a token on the trade screen GUI. While anyone can trade, only the contract owner can pre-define a swap route. Most importantly, traders are able to specify the **maximum price impact percentage** to avoid having large trades resulting in significant price fluctuations.

## Why Do We Need This

This contract can be used by other client contracts such as:

- Dollar-cost-average (DCA)
- Portfolio balancing
- Simulating limit orders
- Trading strategies
- Lending protocols

and many many others.

## Workshop Goals

- Understanding CosmWasm Fundamentals.
- Getting familiar with reply messages.
- Utilizing [osmosis-rust](https://github.com/osmosis-labs/osmosis-rust)
    * [osmosis-std](https://github.com/osmosis-labs/osmosis-rust/tree/main/packages/osmosis-std)
    * [osmosis-testing](https://github.com/osmosis-labs/osmosis-rust/tree/main/packages/osmosis-std)
- Interacting with the Osmosis chain in CosmWasm.
- Learning more about swaps and TWAP.

## Prerequisites

### Option 1: Quick Install with Osmosis Installer

TODO

### Option 2: Manual Install

TODO

## CosmWasm Fundamentals

- [CosmWasm Zero-To-Hero by @Callum-A](https://github.com/Callum-A/cosmwasm-zero-to-hero)
- [CosmWasm Semantics](https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md)
- [Smart Contract Architecture and Common Pitfalls](https://docs.cosmwasm.com/docs/0.16/architecture/smart-contracts)

## Checkpoints

We will over the development of the contract from scratch. The following checkpoints (git branches)
are available to jump to any specific point in the workshop.

- [0. Setup and Contract Boilerplate](https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/0-setup-boilerplate)
- [1. Complete Instantiate Message and Write Out Stubs](https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/1-instantiate-msg-stubs)
- [2. Implement Set Route Message](https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/2-set-route-msg)
- [3. Implement Queries](https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/3-queries)
- [4. Implement Basic Swap Message](https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/4-swap-msg)
- [5. Final Result: Swap with Maximum Price Impact Percentage](https://github.com/p0mvn/swaprouter-workshop)

### 0. Setup and Contract Boilerplate

Goal: have a foundational structure of the CosmWasm contract generated with Beaker.
We will go over this structure and understand the anatomy of a smart contract.

#### Step 1: Let’s generate and build a new CosmWasm project with Beaker

```bash
# Generate workspace
beaker new swaprouter-workshop

# Generate contract inside the workspace
cd swaprouter-workshop
beaker wasm new swaprouter
```

Open workspace root in your editor of choice

You should observe the following directory structure:

```bash
tree
.
├── Beaker.toml
├── Cargo.lock
├── Cargo.toml
├── README.md
├── contracts
│   └── swaprouter
│       ├── Cargo.toml
│       ├── LICENSE
│       ├── NOTICE
│       ├── README.md
│       └── src
│           ├── bin
│           │   └── schema.rs
│           ├── contract.rs
│           ├── error.rs
│           ├── lib.rs
│           ├── msg.rs
│           └── state.rs
└── ts
    └── ...
```

Let's go over the Rust files that we care about:

- `contract.rs`

Defines entrypoints of the smart contract. There are 3 main entrypoints that we will be interacting with
today:

1. `instantiate`

TODO: explain

2. `execute`

TODO: explain

3. `query`

TODO: explain

4. `reply`

TODO: explain

There are other entrypoints such as `migrate` that are outside of scope of this workshop.

- `error.rs`

This file defines the error types that can be returned by the contract. We will be defining custom
errors later in the workshop.

- `msg.rs`

Here, we are going to define all of the messages that our smart contract will support. Each entrypoint
discussed earlier has its own set of messages.

- `state.rs`

Defines the state of the smart contract. In this file, we will be defining the storage layout for
persising any information across contract calls.

TODO: compare to Ethereum, common pitfalls and protection from reentrancy attacks

### 1. Complete Instantiate Message and Write Out Stubs

Goal: finish the implementation of `InstantiateMsg` and outline the stubs for all other messages.

#### User Stories

Let's begin by undrstanding the requirements.

1. As a contract owner, I would like to have exclusive access to set trading routes so that I can be the only one with privileges of limiting trades to tokens needed by my application

Need:

- `InstantiateMsg` that stores the contract owner address
    * Fully implemented in checkpoint this checkpoint - 1

- `ExecuteMsg::SetRoute` that can only be called by the contract owner.
    * Fully implemented in checkpoint 2

- `QueryMsg::GetOwner` to query the owner of the contract.


2. As a contract user, I would like to be able to trade only on the pre-defined route so that I can be confident I am only exposed to the trades needed by the application

Need:

- `ExecuteMsg::Swap` that can be called by anyone.
    * Fully implemented in checkpoint 4
    * Performs a swap on the pre-defined route.

- Swap reply message
    * Fully implemented in checkpoint 4
    * `ExecuteMsg::Swap` requires interacting with the Osmosis chain.
    So we need to send a swap message to it and receive a reply, all in
    one transaction.

TODO: is this the place to dive deeper into reply messages?


3. As a contract user, I would like to be able to trade with maximum price impact so that my large
trades do not affect the market too much.

Need:

- Improve `ExecuteMsg::Swap` to support a new trade type with max price impact.
    * Fully implemented in checkpoint 5 (latest state of the repository).


### 2. Implement Set Route Message

### 3. Implement Queries

### 4. Implement Basic Swap Message

### 5. Final Result: Swap with Maximum Price Impact Percentage
