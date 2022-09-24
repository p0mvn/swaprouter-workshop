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

When contract's code is already uploaded to the chain. It needs to be instantiated to initialize state.
This entypoint takes a respective `InstantiateMsg` and executes it. It is the first message that is run for the contract, and it can only be run once.

2. `execute`

After a contract is initialized with the `InstantiateMsg` we can continue running other defined
execute messages. This entrypoint takes these execute messages and propagates them to the
relevant entrypoint.

3. `query`

There are times when clients need to know the state of the contract. This entrypoint takes
relevant messages so that users can query the state of the contract.

4. `reply`

Due to the architecture necessary to protect against re-entrancy attacks (to be discussed later),
it was originally impossible to receive replies from the messages executes from within the contract.

This entrypoint was later introduced to help with receiving replies. 
Code in the reply entrypoint is comparable to a callback function running after
some asyncronous logic is executed.

For example, in our workshop we are going to send a message to the Osmosis chain to swap some tokens.
We will need to know how many tokens we will receive in return.

Therefore, we will issue a swap message from a relevant `ExecuteMsg` and then receive the
result of the swap in the `reply` entrypoint.

CosmWasm enables this functionality by wrapping a `CosmosMsg` (swap message) into a submessage.
Submessage has a cache context so if it fails, it can rollback any changes that were made earleir
and fail the whole transaction. There are certain edge cases where the submessages do not fail
depending on the kind of the reply handler so please see the following for more info:

https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#Submessages

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

If you get stuck, see: https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/1-instantiate-msg-stubs

#### User Stories

Let's begin by understanding the requirements.

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


3. As a contract user, I would like to be able to trade with maximum price impact so that my large
trades do not affect the market too much.

Need:

- Improve `ExecuteMsg::Swap` to support a new trade type with max price impact.
    * Fully implemented in checkpoint 5 (latest state of the repository).

Based on the above requirements, let's outline all the logic that we require.

Each `QueryMsg` and `ExecuteMsg` need to have a relevant handler function.
Therefore, let's proceed by defining them all.

There are 2 `ExecuteMsg`s - `SetRoute` and `Swap`. So, in `execute.rs` we
define the rough stub handlers for them. Note that they might change as
we progress with the workshop.

```rust
pub fn set_route(
    _input_denom: String,
    _output_denom: String,
    _pool_route: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn swap(
    _input_coin: Coin,
    _output_denom: String,
    _minimum_output_amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
```

Similarly, there are 2 query messages so we define their stub handlers
in `query.rs`:

```rust
pub fn query_owner() -> StdResult<GetOwnerResponse> {
    Ok(GetOwnerResponse {
        owner: String::default(),
    })
}

pub fn query_route(input_denom: String, output_denom: String) -> StdResult<GetRouteResponse> {
    Ok(GetRouteResponse { pool_route: vec![] })
}
```

Now, we are ready to outline the actual messages in `msg.rs`. Note that
upon defining them as below, you will get compilation errors due to the
need to handle these messages in `contract.rs`. We will address that
right after.

```rust
/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    SetRoute {
        input_denom: String,
        output_denom: String,
        pool_route: Vec<SwapAmountInRoute>,
    },
    Swap {
        input_coin: Coin,
        output_denom: String,
        minimum_output_amount: Uint128,
    },
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOwnerResponse)]
    GetOwner {},
    #[returns(GetRouteResponse)]
    GetRoute {
        input_denom: String,
        output_denom: String,
    },
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct GetRouteResponse {
    pub pool_route: Vec<SwapAmountInRoute>,
}
```

With the messaged defined, we can proceed by connecting them to
the handlers in `contract.rs`:

```rust
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRoute {
            input_denom,
            output_denom,
            pool_route,
        } => set_route(input_denom, output_denom, pool_route),
        ExecuteMsg::Swap {
            input_coin,
            output_denom,
            minimum_output_amount,
        } => swap(input_coin, output_denom, minimum_output_amount),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner()?),
        QueryMsg::GetRoute { input_denom, output_denom } => to_binary(&query_route(input_denom, output_denom)?),
    }
}
```

As the last step for the completion of this checkpoint, we are going to fully implement
the `InstantiateMsg`. For that, we need to define the state of the contract for 
persisting the contract owner. In `state.rs` add:

```rust
pub const OWNER: Item<Addr> = Item::new("owner");
```

Go back to `contract.rs` and implement the `instantiate` entrypoint:

```rust
/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner_address = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner_address)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}
```

### 2. Implement Set Route Message

### 3. Implement Queries

### 4. Implement Basic Swap Message

### 5. Final Result: Swap with Maximum Price Impact Percentage
