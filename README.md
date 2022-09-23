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
