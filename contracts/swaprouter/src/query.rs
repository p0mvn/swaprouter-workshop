use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{GetOwnerResponse, GetRouteResponse},
    state::{OWNER, ROUTING_TABLE},
};


// query_owner returns contracr owner. Returns error on storage failure.
pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: owner.into_string(),
    })
}

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
