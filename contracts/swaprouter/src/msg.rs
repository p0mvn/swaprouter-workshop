use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Uint128};
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
        swap_type: SwapType,
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

#[cw_serde]
pub enum SwapType {
    MaxPriceImpactPercentage(Decimal),
    MinOutputAmount(Uint128),
}
