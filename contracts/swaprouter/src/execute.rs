use cosmwasm_std::{Coin, Response, Uint128};
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

use crate::ContractError;

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
