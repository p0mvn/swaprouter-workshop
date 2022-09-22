use cosmwasm_std::{StdResult, Deps};

use crate::{msg::{GetOwnerResponse, GetRouteResponse}, state::{OWNER, ROUTING_TABLE}};

pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: owner.into_string(),
    })
}

pub fn query_route(deps: Deps, input_denom: String, output_denom: String) -> StdResult<GetRouteResponse> {
    let route = ROUTING_TABLE.load(deps.storage, (&input_denom, &output_denom))?;
    Ok(GetRouteResponse { pool_route: route })
}
