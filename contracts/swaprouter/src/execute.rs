use cosmwasm_std::{coin, has_coins, Coin, DepsMut, Env, MessageInfo, Response, SubMsg};
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

use crate::contract::SWAP_REPLY_ID;
use crate::helpers::{
    calculate_min_output_from_twap, generate_swap_msg, validate_is_contract_owner,
    validate_pool_route,
};
use crate::msg::SwapType;
use crate::state::{SwapMsgReplyState, ROUTING_TABLE, SWAP_REPLY_STATES};
use crate::ContractError;

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
pub fn set_route(
    deps: DepsMut,
    info: MessageInfo,
    input_denom: String,
    output_denom: String,
    pool_route: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    validate_is_contract_owner(deps.as_ref(), info.sender)?;

    validate_pool_route(
        deps.as_ref(),
        input_denom.clone(),
        output_denom.clone(),
        pool_route.clone(),
    )?;

    ROUTING_TABLE.save(deps.storage, (&input_denom, &output_denom), &pool_route)?;

    Ok(Response::new().add_attribute("action", "set_route"))
}

// swap initiates an Osmosis swap message of the input_coin to at least
// minimum_output_token of another coin. Wraps the message into
// CosmWasm swap message to receive reply from the respective entrypoint.
// Returns error if:
// - funds sent in by the initiator do no match the input_coin.
// - fails to generate the message.
pub fn swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    input_coin: Coin,
    output_denom: String,
    swap_type: SwapType,
) -> Result<Response, ContractError> {
    if !has_coins(&info.funds, &input_coin) {
        return Err(ContractError::InsufficientFunds {});
    }

    // get minimum output coin from swap type.
    let minimum_output_token = match swap_type {
        SwapType::MaxPriceImpactPercentage(percentage) => calculate_min_output_from_twap(
            deps.as_ref(),
            input_coin.clone(),
            output_denom,
            env.block.time,
            percentage,
        )?,
        SwapType::MinOutputAmount(minimum_output_amount) => {
            coin(minimum_output_amount.u128(), output_denom)
        }
    };

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
        .add_submessage(SubMsg::reply_on_success(swap_msg, SWAP_REPLY_ID)))
}
