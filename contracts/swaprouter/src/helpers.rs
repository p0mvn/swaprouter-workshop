use cosmwasm_std::{Addr, Deps};
use osmosis_std::types::osmosis::gamm::v1beta1::{
    QueryTotalPoolLiquidityRequest, SwapAmountInRoute,
};

use crate::{state::OWNER, ContractError};

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
