use cosmwasm_std::{Coin, DepsMut, MessageInfo, Response, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

use crate::helpers::{validate_is_contract_owner, validate_pool_route};
use crate::state::ROUTING_TABLE;
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

pub fn swap(
    _input_coin: Coin,
    _output_denom: String,
    _minimum_output_amount: Uint128,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
