use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    SubMsgResponse, SubMsgResult, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountInResponse;

use crate::error::ContractError;
use crate::execute::{set_route, swap};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_owner, query_route};
use crate::state::{SwapMsgReplyState, OWNER, SWAP_REPLY_STATES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swaprouter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Msg Reply IDs
pub const SWAP_REPLY_ID: u64 = 1u64;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner_address = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner_address)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRoute {
            input_denom,
            output_denom,
            pool_route,
        } => set_route(deps, info, input_denom, output_denom, pool_route),
        ExecuteMsg::Swap {
            input_coin,
            output_denom,
            swap_type,
        } => swap(deps, env, info, input_coin, output_denom, swap_type),
    }
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetRoute {
            input_denom,
            output_denom,
        } => to_binary(&query_route(deps, input_denom, output_denom)?),
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == SWAP_REPLY_ID {
        // get intermediate swap reply state. Error if not found.
        let swap_msg_state = SWAP_REPLY_STATES.load(deps.storage, msg.id)?;

        // prune intermedate state
        SWAP_REPLY_STATES.remove(deps.storage, msg.id);

        // call reply function to handle the swap return
        handle_swap_reply(msg, swap_msg_state)
    } else {
        Ok(Response::new())
    }
}

// handle_swap_reply deserializes the response from Osmosis chain
// If the response is successful and swap is complete, send
// the swapped token to the original user who initiated the swap.
// Otherwise, return contract error.
pub fn handle_swap_reply(
    msg: Reply,
    swap_msg_reply_state: SwapMsgReplyState,
) -> Result<Response, ContractError> {
    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
        // Unwrap and deserialize message response.
        let res: MsgSwapExactAmountInResponse = b.try_into().map_err(ContractError::Std)?;

        // Retrieve swapped amount.
        let amount = Uint128::from_str(&res.token_out_amount)?;

        // Retrieve swapped denom from reply state.
        let send_denom = &swap_msg_reply_state
            .swap_msg
            .routes
            .last()
            .unwrap()
            .token_out_denom;

        // Send the swapped token from contract to the original
        // user who initiated the swap.
        let bank_msg = BankMsg::Send {
            to_address: swap_msg_reply_state.original_sender.into_string(),
            amount: coins(amount.u128(), send_denom),
        };

        // Success response.
        return Ok(Response::new()
            .add_message(bank_msg)
            // This attribute should be present in the reply events.
            .add_attribute("token_out_amount", amount));
    }

    Err(ContractError::FailedSwap {
        reason: msg.result.unwrap_err(),
    })
}

#[cfg(test)]
mod tests {
    use crate::msg::GetOwnerResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn instantiate_works() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            owner: String::from(MOCK_CONTRACT_ADDR),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // make sure that the owner was set correctly.
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: GetOwnerResponse = from_binary(&res).unwrap();
        assert_eq!(MOCK_CONTRACT_ADDR, value.owner);
    }
}
