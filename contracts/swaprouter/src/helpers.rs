use std::ops::{Div, Mul};

use cosmwasm_std::{Addr, Coin, Decimal, Deps, Timestamp, Uint128};
use osmosis_std::shim::Timestamp as OsmosisTimestamp;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgSwapExactAmountIn, QueryTotalPoolLiquidityRequest, SwapAmountInRoute,
};
use osmosis_std::types::osmosis::twap::v1beta1::TwapQuerier;

use crate::{
    state::{OWNER, ROUTING_TABLE},
    ContractError,
};

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

        twap_price =
            twap_price
                .checked_mul(current_twap.into())
                .map_err(|_e| ContractError::CustomError {
                    val: format!("Invalid value for twap price: {twap_price} * {twap}"),
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
