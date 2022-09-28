# Osmosis Swaprouter Worskhop

This is a workshop for building an Osmosis Swap Router CosmWasm contract.

The original contract repository is located here:
https://github.com/osmosis-labs/swaprouter

Original authors:
- [sunnya97](https://github.com/sunnya97)
- [nicolaslara](https://github.com/nicolaslara)
- [iboss-ptk](https://github.com/iboss-ptk)

## What Is This

A contract that allows to swap an **exact** amount of tokens for a **minimum** of another token, similar to swapping a token on the trade screen GUI. While anyone can trade, only the contract owner can pre-define a swap route. Most importantly, traders are able to specify the **maximum slippage percentage** to avoid having large trades resulting in significant price fluctuations.

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
- Getting familiar with reply message.
- Utilizing [osmosis-rust](https://github.com/osmosis-labs/osmosis-rust)
    * [osmosis-std](https://github.com/osmosis-labs/osmosis-rust/tree/main/packages/osmosis-std)
    * [osmosis-testing](https://github.com/osmosis-labs/osmosis-rust/tree/main/packages/osmosis-std)
- Interacting with the Osmosis chain in CosmWasm.
- Learning more about swaps and TWAP.

## Prerequisites: Rust, Contract Environment, Beaker, and Osmosis

Before begining, you must set up Rust, your contract environment, Beaker, and Osmosis with one of the two following options:

### Option 1: Automatic Setup

Start the installer with the following command, choose LocalOsmosis (option 3), and follow the prompts:

```bash
bash <(curl -sL https://get.osmosis.zone/run)
```

### Option 2: Manual Setup

#### Rust

Install Rust using rustup with the following command and follow the prompts:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Contact Environment

Set 'stable' as the default release channel:

```bash
rustup default stable
```

Add WASM as the compilation target:

```bash
rustup target add wasm32-unknown-unknown
```

Install the following packages to generate the contract:

```bash
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-scrip
```

#### Beaker

Install Beaker with the following command:

```bash
cargo install -f beaker
```

#### Osmosis

Setup v12.x Osmosis

```bash
cd $HOME
git clone https://github.com/osmosis-labs/osmosis.git
cd $HOME/osmosis
git checkout v12.x
make install
source ~/.profile
```

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
- [5. Final Result: Swap with Maximum Slippage Percentage](https://github.com/p0mvn/swaprouter-workshop)

## FAQ

- How do I add a dependency to my contract?
    * Update contract's `Cargo.toml`
    * [Example](https://github.com/p0mvn/swaprouter-workshop/blob/main/contracts/swaprouter/Cargo.toml)

- What versions of `osmosis_std` and `osmosis_rust` should I use?
    * `osmosis-std = {git = "https://github.com/osmosis-labs/osmosis-rust", branch = "osmosis-v12-rc2"}`
    * `osmosis-testing = {git = "https://github.com/osmosis-labs/osmosis-rust", branch = "main"}`

- What version of Beaker should I use?
    * [v0.1.0](https://github.com/osmosis-labs/beaker/releases/tag/v0.1.0)

- What is the version of `Osmosis/LocalOsmosis`?
    * [`v12.x`](https://github.com/osmosis-labs/osmosis/tree/v12.x/tests/localosmosis)

### 0. Setup and Contract Boilerplate

**Goals**:
- Have a foundational structure of the CosmWasm contract generated with Beaker.
- Understand the anatomy of a smart contract.
- Learn about the core architecture and security benefits.

#### Generate a new CosmWasm Project with Beaker

First, generate the swaprouter workspace with the following command:

```bash
beaker new swaprouter-workshop
```

Select the minimal template. Now generate a contract inside the workspace.

```bash
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

#### Anatomy of a CosmWasm Contract

Let's go over the Rust files that we care about:

- `contract.rs`

Defines entrypoints of the smart contract. There are 3 main entrypoints that we will be interacting with
today:

1. `instantiate`

Once the contract's code is uploaded to the chain, it needs to be instantiated to initialize state.
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
depending on the kind of the reply handler so please read [this CosmWasm SEMANTICS page](https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#Submessages) if you are interested to learn more.

Key points to note from the additional reading:

> Submessages (and their replies) are all executed before any messages. They also follow the depth first rules as with messages. Here is a simple example. Contract A returns submessages S1 and S2, and message M1. Submessage S1 returns message N1. The order will be: S1, N1, reply(S1), S2, reply(S2), M1.

> We do not drop reply data. If execute returns `data: Some(b"try 1")` and reply returns `data:  Some(b"try 2"), we return , reply will overwrite the data from execute.

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


#### Security Benefits

- **Private internal state**
   * In contrast to cosmos-sdk, where each module has read and write access to another module's store

- **Serialized execution**
   * Comparable to an automatic mutex over the contract code. Protects from re-entrancy attacks.

- **Atomic Executiion**
   * If a submessage fails, the whole transaction fails and the state is rolled back.

A lot of these benefits are guaranteed by the "Actor Framework" architecture employed by CosmWasm.

So what is this? The Actor is a single instantiation of a contract, which can perform several actions. When the actor finishes his job, he prepares a summary of it, which includes the list of things that have to be done, to complete the whole scheduled task.

KFC analogy:
- The restaurant (an actor)
- Me making an order to the cashier of the restaurant (action)
- The cashier takes an order (execute message) and tells the cook what to make (issue sub-message)
- The cook notifies cashier once an order is ready (reply message)
- If the cook does not have ingridients, my entire order is aborted like it never hapened (atomic execution)
- Things happen in sequence
- I don't have access to the internal state of the restaurant unless exposed by queries (private internal state). 

Sources:
- [Security Benefits](https://docs.cosmwasm.com/docs/1.0/architecture/actor#security-benefits)
- [Actor Framework](https://docs.cosmwasm.com/docs/1.0/actor-model/idea/)
- [Comparison with Solidity](https://docs.cosmwasm.com/docs/1.0/architecture/smart-contracts/)

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.

### 1. Complete Instantiate Message and Write Out Stubs

**Goals**:
- Finish the implementation of `InstantiateMsg` and outline the stubs for all other messages.

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


3. As a contract user, I would like to be able to trade with maximum slippage so that my large
trades do not affect the market too much.

Need:

- Improve `ExecuteMsg::Swap` to support a new trade type with max slippage.
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
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

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

Note that in the previous step we brought in the following struct into the scope:
```rust
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;
```

Essentially, this translates directly into the Osmosis proto definition that we use on chain.
We import it from the `osmosis_std` crate of the `osmosis-rust` repository.

By having these interoperable struct defintions, we will be able to easily call into the Osmosis
chain messages to perform a swap later in the workshop.

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

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.

### 2. Implement Set Route Message

**Goals**:
- Fully-functional `SetRoute` messages. Short intro to `osmosis-testing`. 

If you get stuck, see: https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/2-set-route-msg

Completing the implementation is as simple as filling in the blank stub of the
`set_route` handler in `execute.rs` that we created in the previous checkpoint.

Let's begin by defining the function spec to understand what we need to do.
In `execute.rs`:

```rust
// set_route sets route for swaps. Only contract owner may execute this message.
// Returns response with attributes on success.
// Errors if:
// - executed by anyone other than the owner
// - invalid pool route given
//
// Example 1 (one-hop):
// OSMO -> ATOM
// input: OSMO
// OUTPUT: ATOM
// ROUTE = [ { pool_id: 1, token_out_denom: ATOM } ]
//
// Example 2 (multi-hop):
// OSMO -> ATOM -> STAKE
// input: OSMO
// OUTPUT: ATOM
// ROUTE = [ { pool_id: 1, token_out_denom: ATOM }, { pool_id: 2, token_out_denom: STAKE } ]
pub fn set_route(...) {
    ...
}
```

So we need to store the route in the contract state. Let's define it in `state.rs`:

```rust
pub const ROUTING_TABLE: Map<(&str, &str), Vec<SwapAmountInRoute>> = Map::new("routing_table");
```

Now, we are ready to dive into the implementation of `set_route`:

```rust
pub fn set_route(
    deps: DepsMut,
    info: MessageInfo,
    input_denom: String,
    output_denom: String,
    pool_route: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    // Make sure that sender is contract owner.
    validate_is_contract_owner(deps.as_ref(), info.sender)?;

    // Validate that pool route is valid.
    validate_pool_route(
        deps.as_ref(),
        input_denom.clone(),
        output_denom.clone(),
        pool_route.clone(),
    )?;

    // Save the route to the routing table.
    ROUTING_TABLE.save(deps.storage, (&input_denom, &output_denom), &pool_route)?;

    // Response of success. 
    Ok(Response::new().add_attribute("action", "set_route"))
}
```

Implement each helper in `helpers.rs`:

```rust
// validate_is_contract_owner validates if sender is the contract owner.
// Returns success if sender is the owner, error otherwise.
pub fn validate_is_contract_owner(deps: Deps, sender: Addr) -> Result<(), ContractError> {
    let owner = OWNER.load(deps.storage).unwrap();
    if owner != sender {
        Err(ContractError::Unauthorized {})
    } else {
        Ok(())
    }
}

// validate_pool_route validates if the pool route is valid.
// Returns success if it is, error otherwise.
pub fn validate_pool_route(
    deps: Deps,
    input_denom: String,
    output_denom: String,
    pool_route: Vec<SwapAmountInRoute>,
) -> Result<(), ContractError> {
    let mut current_denom_in = input_denom;

    // Iterate over each route
    for route_part in &pool_route {
        // Query liqudity of the pool id specified by the route
        // from the Osmosis chain.
        let liquidity = QueryTotalPoolLiquidityRequest {
            pool_id: route_part.pool_id,
        }
        .query(&deps.querier)?
        .liquidity;

        // If the current denom to swap in does not match any of the denoms
        // in the pool, return error.
        if !liquidity.iter().any(|coin| coin.denom == current_denom_in) {
            return Result::Err(ContractError::InvalidPoolRoute {
                reason: format!(
                    "denom {} is not in pool id {}",
                    current_denom_in, route_part.pool_id
                ),
            });
        }

        // If the denom to swap out does not match any of the denoms in the pool,
        // return error.
        if !liquidity
            .iter()
            .any(|coin| coin.denom == route_part.token_out_denom)
        {
            return Result::Err(ContractError::InvalidPoolRoute {
                reason: format!(
                    "denom {} is not in pool id {}",
                    current_denom_in, route_part.pool_id
                ),
            });
        }

        // The denom to swap in for the next route is the denom
        // out for the current route.
        current_denom_in = route_part.token_out_denom.clone();
    }

    // Make sure the final route output asset is the same as the expected output_denom
    if current_denom_in != output_denom {
        return Result::Err(ContractError::InvalidPoolRoute {
            reason: "last denom doesn't match".to_string(),
        });
    }

    Ok(())
}
```

Now, we are ready to test this message with `osmosis-testing`. We omit listing details in this README.
However, there are 2 relevant files on the checkpoint 2 branch:

- [`set_route_test.rs`](https://github.com/p0mvn/swaprouter-workshop/blob/checkpoint/2-set-route-msg/contracts/swaprouter/tests/set_route_test.rs)
    * Actual tests
- [`test_env.rs`](https://github.com/p0mvn/swaprouter-workshop/blob/checkpoint/2-set-route-msg/contracts/swaprouter/tests/test_env.rs)
    * Setup logic

Essentially, `osmosis-testing` spins up an actual test Osmosis application in the background. That allows us
to realistically check that all of the messages are functioning as expected contrary to the original `cw_multitest` approach
that forces users to define mocks.

With these files and `osmosis_testing` added to your `Cargo.toml`, you can run:
- `cargo wasm` to build the contract
- `cargo test` to run the osmosis tests

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.
- [x] Be able to `cargo test` without any errors.

### 3. Implement Queries

**Goals**:
- Implement queries and write up basic test for `InstantiateMsg` by utilizing the queries.

If you get stuck, see: https://github.com/p0mvn/swaprouter-workshop/tree/checkpoint/3-queries

Let's beging by going to the earlier created `query.rs` where we have 2 stubs.

- `query_owner` - needs to return the owner from the state

```rust
// query_owner returns contracr owner. Returns error on storage failure.
pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: owner.into_string(),
    })
}
```

- `query_route` - returns query route from the state given denoms.

```rust
// query_route returns query route for given
// input and output denoms.
// Returns error on any storage failure.
pub fn query_route(
    deps: Deps,
    input_denom: String,
    output_denom: String,
) -> StdResult<GetRouteResponse> {
    let route = ROUTING_TABLE.load(deps.storage, (&input_denom, &output_denom))?;
    Ok(GetRouteResponse { pool_route: route })
}
```

Query implementations are now completed.

Let'go back to `contract.rs` and write a basic unit test for the `InstantiateMsg`
by utilizing `query_owner`. At the bottom of the file add:

```rust
#[cfg(test)]
mod tests {
    use crate::msg::GetOwnerResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            owner: String::from(MOCK_CONTRACT_ADDR),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // make sure that the owner was set correctly.
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!(MOCK_CONTRACT_ADDR, value.owner);
    }
}
```

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.
- [x] Be able to `cargo test` without any errors.

### 4. Implement Basic Swap Message

**Goals**:
- Understand the semantics of the single-asset swap in Osmosis
- Implement `ExecuteMsg::Swap` that performs a basic `SwapExactAmountIn` swap (w/o slippage)
- Imp
- Learn how to interact with the Osmosis chain from CosmWasm 

If you get stuck, see: https://github.com/p0mvn/swaprouter-workshop/blob/checkpoint/4-swap-msg

#### Swap Message Semantics

Before we proceed with the implementation, let's take a step back and understand the semantics 
of the swap that we are going to work with.

As it stands today, Osmosis is a constant product function AMM based on the balancer design.

We are going to focus on the [`SwapExactAmountIn`](https://docs.osmosis.zone/osmosis-core/modules/spec-gamm/#swap-exact-amount-in) swap. The semantics of this swap are as follows:

> Swap an **exact** amount of tokens for a **minimum** of another token.

We are going to issue [this message](https://github.com/osmosis-labs/osmosis/blob/18d70da2a881f3a938975d7cc55a9107edef6212/proto/osmosis/gamm/v1beta1/tx.proto#L68-L80) to the Osmosis chain.

Again, `osmosis_std`'s proto bindings are going to help us with that.

#### Implementation

As discussed previously, we will need to utilize the reply
entrypoint and CosmWasm submessages in order to receive the
swap reply from the Osmosis chain, all trigerred by `ExecuteMsg::Swap`.

- **`ExecuteMsg::Swap`**

Similarly to `ExecuteMsg::SetRoute`, let's begin by understanding
what we want our swap handler to do.

```rust
// swap initiates an Osmosis swap message of the input_coin to at least
// minimum_output_token of another coin. Wraps the message into
// CosmWasm swap message to receive reply from the respective entrypoint.
// Returns error if:
// - funds sent in by the initiator do no match the input_coin.
// - fails to generate the message.
pub fn swap(...) {
    // check if the sender has funds equal to input coind

    // generate the Osmosis swap message

    // persist some intermediate state to later receive in the reply entrypoint

    // return success with added swap submessage.
    // this submessage will get propagated to the reply entrypoint.
}
```

Awesome! Note that we will need some intermediary state persisted
between the swap caller and the reply entrypoint. This is because
the Osmosis chain does not return all of the inputs to the swap message.

So, in `state.rs.` we add:

```rust
#[cw_serde]
pub struct SwapMsgReplyState {
    pub original_sender: Addr,
    pub swap_msg: MsgSwapExactAmountIn,
}

...
// SWAP_REPLY_STATES persists data from swap message creation until the reply receipt.
pub const SWAP_REPLY_STATES: Map<u64, SwapMsgReplyState> = Map::new("swap_reply_states");
```

Let's now translate the swam handler comments into code in `execute.rs`

```rust
pub fn swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    input_coin: Coin,
    minimum_output_token: Coin,
) -> Result<Response, ContractError> {
    if !has_coins(&info.funds, &input_coin) {
        return Err(ContractError::InsufficientFunds {});
    }

    // generate the swap message using osmosis-rust (osmosis_std).
    let swap_msg = generate_swap_msg(
        deps.as_ref(),
        env.contract.address,
        input_coin,
        minimum_output_token,
    )?;

    // save intermediate state for reply
    SWAP_REPLY_STATES.save(
        deps.storage,
        SWAP_REPLY_ID,
        &SwapMsgReplyState {
            original_sender: info.sender,
            swap_msg: swap_msg.clone(),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "swap")
        // add sub message with reply on success. See reply entrypoint for the continuation of the flow.
        .add_submessage(SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID))) // SWAP_REPLY_ID is defined in `contract.rs` in future steps
}
```

While `has_coins` is a helper that can be imported from `cosmwasm_std`,
we need to define `generate_swap_msg` ourselves in `helpers.rs`:

```rust
// generate_swap_msg generates and returns an Osmosis
// MsgSwapExactAmountIn with sender, input token and min_output_token.
// Returns error if there is no supported route
// between input_token and min_output_token.
pub fn generate_swap_msg(
    deps: Deps,
    sender: Addr,
    input_token: Coin,
    min_output_token: Coin,
) -> Result<MsgSwapExactAmountIn, ContractError> {
    // get trade route
    let route = ROUTING_TABLE.load(deps.storage, (&input_token.denom, &min_output_token.denom))?;

    Ok(MsgSwapExactAmountIn {
        sender: sender.into_string(),
        routes: route,
        token_in: Some(input_token.into()),
        token_out_min_amount: min_output_token.amount.to_string(),
    })
}
```

- **Implement Reply Entrypoint**

Hopefully, by now you have a good understanding of why we need the reply entrypoint.
Now, we are going to discuss the implementation.

While in our contract, we only have one reply message, for bigger contracts, it is well possible to have several.
Therefore, the first thing that we need to do is check that we receive the desired reply message and propagate
it to the appropriate handler. In `contract.rs`:

```rust
// Msg Reply IDs
pub const SWAP_REPLY_ID: u64 = 1u64;

...

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == SWAP_REPLY_ID {
        // get intermediate swap reply state. Error if not found.
        let swap_msg_state = SWAP_REPLY_STATES.load(deps.storage, msg.id)?;

        // prune intermedate state
        SWAP_REPLY_STATES.remove(deps.storage, msg.id);

        // call reply function to handle the swap return
        handle_swap_reply(msg, swap_msg_state)
    } else {
        Ok(Response::new())
    }
}
```

Right below the `reply` function, we define `handle_swap_reply` handler.
Let's begin with the stub and comments.

```rust
// handle_swap_reply deserializes the response from Osmosis chain
// If the response is successful and swap is complete, send
// the swapped token to the original user who initiated the swap.
// Otherwise, return contract error.
pub fn handle_swap_reply(
    msg: Reply,
    swap_msg_reply_state: SwapMsgReplyState,
) -> Result<Response, ContractError> {
    // If response can be deserialized

    //    1. Retrieve swapped amount from deserizlized response
    
    //    2. Retrieve swapped denom from reply state we created earleir

    //    3. Send swapped token to original sender from contract.

    //    4. Return success with attributes

    // if cannot deserialize
    //     return error
}
```

By translating comments into code, we get:

```rust
pub fn handle_swap_reply(
    msg: Reply,
    swap_msg_reply_state: SwapMsgReplyState,
) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        // Unwrap and deserialize message response.
        let res: MsgSwapExactAmountInResponse = b.try_into().map_err(ContractError::Std)?;

        // Retrieve swapped amount.
        let amount = Uint128::from_str(&res.token_out_amount)?;

        // Retrieve swapped denom from reply state.
        let send_denom = &swap_msg_reply_state
            .swap_msg
            .routes
            .last()
            .unwrap()
            .token_out_denom;

        // Send the swapped token from contract to the original
        // user who initiated the swap.
        let bank_msg = BankMsg::Send {
            to_address: swap_msg_reply_state.original_sender.into_string(),
            amount: coins(amount.u128(), send_denom),
        };

        // Success response.
        return Ok(Response::new()
            .add_message(bank_msg)
            // This attribute should be present in the reply events.
            .add_attribute("token_out_amount", amount));
    }

    Err(ContractError::FailedSwap {
        reason: msg.result.unwrap_err(),
    })
}
```

Checkpoint 4 should now be complete. At this point, we have all the core
functionality layed out. The remaining logic is adding slippage limit
functionality to the contract.

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.
- [x] Be able to `cargo test` without any errors.

### 5. Final Result: Swap with Maximum Slippage Percentage

**Goals**:
- Understand and utilize TWAP.
- Implement swap with the max slippage.

If you get stuck, see: https://github.com/p0mvn/swaprouter-workshop/tree/main

#### What is TWAP

TWAP - Time Weighted Average Price. It is a price feed that provides smart contracts
with prices that are **manipulation resistant**.

The details are outside of scope of this tutorial but, basically, we don't want to use
real-time prices for security reasons. Instead, we use the time weighted prices
that are more difficult to manipulate.

More information about this can be found [here](https://soliditydeveloper.com/uniswap-oracle)

#### What Are Slippage and Price Impact

Slippage refers to the change in price caused by the broad market
movements.

On the other hand, price impact is how much your sell/buy directly impacts the liquidity pool.

Technically, we are working  with price impact throughout the contract. However, if we look at this abstraction from the user's perspective, they only care about the difference
in price that they are getting broadly. Therefore, we call it slippage. 

For some of the applications such as collateralized loans, it is common to
require to execute large trades. To avoid impacting the market, these
applications might want to set a maximum price impact (slippage) ratio.

#### Implementation

Implementation is about refactoring our `ExecuteMsg::Swap` to now support
an additional trade swap type.

Since this change is a large refactor, it might be easier to refer to the
diff in [this pull-request](https://github.com/p0mvn/swaprouter-workshop/pull/21/files) 

Let's represent the additional swap type by updating the message in `msg.rs`:

```diff
#[cw_serde]
pub enum ExecuteMsg {
    ...
    Swap {
        input_coin: Coin,
        output_denom: String,
-        minimum_output_amount: Uint128,
+        swap_type: SwapType,
    },
}

+#[cw_serde]
+pub enum SwapType {
+    MaxSlippagePercentage(Decimal),
+    MinOutputAmount(Uint128),
+}
```

There are more updates that we will need to do in `contracts.rs` that are omitted from the guide.
Again, refer to [this pull-request](https://github.com/p0mvn/swaprouter-workshop/pull/21/files) for details.

In the meantime, we will move directly to updating the `swap` handler in `execute.rs`:

```diff
pub fn swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    input_coin: Coin,
-    minimum_output_token: Coin,
+    output_denom: String,
+    swap_type: SwapType,
) -> Result<Response, ContractError> {
    if !has_coins(&info.funds, &input_coin) {
        return Err(ContractError::InsufficientFunds {});
    }

+    // get minimum output coin from swap type.
+    let minimum_output_token = match swap_type {
+        SwapType::MaxSlippagePercentage(percentage) => calculate_min_output_from_twap(
+            deps.as_ref(),
+            input_coin.clone(),
+            output_denom,
+            env.block.time,
+            percentage,
+        )?,
+        SwapType::MinOutputAmount(minimum_output_amount) => {
+            coin(minimum_output_amount.u128(), output_denom)
+        }
+    };

    ...
}
```

Lastly, let's implement `calculate_min_output_from_twap` function in `helpers.rs`
for the new swap type:

TODO: break down the implementation more

```rust
pub fn calculate_min_output_from_twap(
    deps: Deps,
    input_token: Coin,
    output_denom: String,
    now: Timestamp,
    percentage_impact: Decimal,
) -> Result<Coin, ContractError> {
    // get trade route
    let route = ROUTING_TABLE.load(deps.storage, (&input_token.denom, &output_denom))?;
    if route.is_empty() {
        return Err(ContractError::InvalidPoolRoute {
            reason: format!("No route foung for {} -> {output_denom}", input_token.denom),
        });
    }

    let percentage = percentage_impact.div(Uint128::new(100));

    let mut twap_price: Decimal = Decimal::one();

    // When swapping from input to output, we need to quote the price in the input token
    // For example when selling uosmo to buy uion:
    // price of <out> is X<in> (i.e.: price of uion is X uosmo)
    let mut quote_denom = input_token.denom;

    let start_time = now.minus_seconds(1);
    let start_time = OsmosisTimestamp {
        seconds: start_time.seconds() as i64,
        nanos: 0_i32,
    };

    for route_part in route {
        deps.api.debug(&format!("route part: {route_part:?}"));

        let twap = TwapQuerier::new(&deps.querier)
            .arithmetic_twap_to_now(
                route_part.pool_id,
                route_part.token_out_denom.clone(), // base_asset
                quote_denom.clone(),                // quote_asset
                Some(start_time.clone()),
            )?
            .arithmetic_twap;

        deps.api.debug(&format!("twap = {twap}"));

        let current_twap: Decimal = twap.parse().map_err(|_e| ContractError::CustomError {
            val: "Invalid twap value received from the chain".to_string(),
        })?;

        twap_price = twap_price.checked_mul(current_twap.into()).map_err(|_e| {
            ContractError::CustomError {
                val: format!("Invalid value for twap price: {twap_price} * {twap}"),
            }
        })?;

        // the current output is the input for the next route_part
        quote_denom = route_part.token_out_denom;
    }

    twap_price = twap_price - twap_price.mul(percentage);
    deps.api.debug(&format!(
        "twap_price minus {percentage_impact}%: {twap_price}"
    ));

    let min_out: Uint128 = input_token.amount.mul(twap_price);
    deps.api.debug(&format!("min: {min_out}"));

    Ok(Coin::new(min_out.into(), output_denom))
}
```

Now, we are ready to test this message with `osmosis-testing`. We omit listing details in this README.
However, there are 2 relevant files on the checkpoint 2 branch:

- [`set_route_test.rs`](https://github.com/p0mvn/swaprouter-workshop/blob/main/contracts/swaprouter/tests/swap_test.rs)
    * Actual tests
- [`test_env.rs`](https://github.com/p0mvn/swaprouter-workshop/blob/main/contracts/swaprouter/tests/test_env.rs)
    * Setup logic

#### Acceptance Criteria

- [x] Be able to `cargo wasm` without any errors.
- [x] Be able to `cargo test` without any errors.
- [x] Be able to deploy and manually test the contract with Beaker (discussed in the next section)

## Deployment

Finally, let's upload our contract and test it with LocalOsmosis and Beaker.

### Start LocalOsmosis

LocalOsmosis is a tool to spin up a local test Osmosis chain.
It is located inside the Osmosis repository and can be interacted
with by Makefile commands.

```bash
cd $HOME/osmosis
git checkout v12.x

# Build localnet Osmosis image.
make localnet-build

# Start LocalOsmosis with test state.
make localnet-start-with-state

# Add test account keys to your keyring.
make localnet-keys

# To stop LocalOsmosis
make localnet-stop 

# To reset the state (only call when chain is stopped)
# Warning: deletes all the progress deployed to chain so far.
make localnet-clean
```

### Deploy to LocalOsmosis

#### Beaker

```bash
# Create a key that we are going to operate with.
beaker key set lo-test1 "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius"

# Deploy swaprouter contract. Owner is the address of an account that we created above.
# N.B. Label is needed in case you happen to want to make a change and redeploy a new version of the contract.
# Since contracts are immutable, you would increase the label by 1 and redeploy to operate on a new version.
beaker wasm deploy swaprouter --signer-account test1 --no-wasm-opt --raw '{ "owner": "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks" }' --label 1
```

#### Osmosisd

- Store contract code

```bash
TX=$(osmosisd tx wasm store target/wasm32-unknown-unknown/release/swaprouter.wasm --from lo-test1 --keyring-backend test --chain-id=localosmosis --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -b block --output json -y | jq -r '.txhash')
CODE_ID=$(osmosisd query tx $TX --output json | jq -r '.logs[0].events[-1].attributes[0].value')
echo "Your contract code_id is $CODE_ID"
```

- Instantiate contract
```bash
osmosisd tx wasm instantiate $CODE_ID '{ "owner": "osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks" }' --from lo-test1 --keyring-backend test --amount 50000uosmo  --label "SwapRouter Contract" --from lo-test1 --chain-id localosmosis --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -b block -y --no-admin
```

- Get Contract Address
```bash
CONTRACT_ADDR=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')

echo $CONTRACT_ADDR
```

### Issue `ExecuteMsg::SetRoute`

```bash
beaker wasm execute swaprouter --raw '{ "set_route": { "input_denom": "stake", "output_denom": "uion", "pool_route": [ { "pool_id": 1, "token_out_denom": "uosmo" }, { "pool_id": 2, "token_out_denom": "uion" } ] } }' --signer-account test1 --label 1
```

### Use Beaker to issue `ExecuteMsg::Swap` with `SwapType::MinOutputAmount`

```bash
beaker wasm execute swaprouter --raw '{"swap": { "input_coin": { "amount": "50", "denom": "stake" }, "output_denom": "uion", "swap_type": { "min_output_amount": "1" } } }' --signer-account test1 --label 1 --gas "100000uosmo" --gas-limit 10000000 --funds "50stake"
```

### Use Beaker to issue `ExecuteMsg::Swap` with `SwapType::MaxSlippagePercentage`

```bash
beaker wasm execute swaprouter --raw '{"swap": { "input_coin": { "amount": "1000", "denom": "stake" }, "output_denom": "uion", "swap_type": { "max_slippage_percentage": "20" } } }' --signer-account test1 --label 1 --gas "100000uosmo" --gas-limit 25000000 --funds "1000stake"
```

### Optional TODO: Create a test contract

TODO: add link to docs page abou what's going on under the hood of beaker deploy

TODO: FAQ for debugging logs
