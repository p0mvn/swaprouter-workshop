use cosmwasm_std::StdResult;

use crate::msg::{GetOwnerResponse, GetRouteResponse};

pub fn query_owner() -> StdResult<GetOwnerResponse> {
    Ok(GetOwnerResponse {
        owner: String::default(),
    })
}

pub fn query_route(input_denom: String, output_denom: String) -> StdResult<GetRouteResponse> {
    Ok(GetRouteResponse { pool_route: vec![] })
}
