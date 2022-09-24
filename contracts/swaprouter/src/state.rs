// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};

#[cw_serde]
pub struct SwapMsgReplyState {
    pub original_sender: Addr,
    pub swap_msg: MsgSwapExactAmountIn,
}

// OWNER stores the contract owner configured at instantiation time.
pub const OWNER: Item<Addr> = Item::new("owner");
// ROUTING_TABLE stores the swap route set by the owner.
pub const ROUTING_TABLE: Map<(&str, &str), Vec<SwapAmountInRoute>> = Map::new("routing_table");
// SWAP_REPLY_STATES persists data from swap message creation until the reply receipt.
pub const SWAP_REPLY_STATES: Map<u64, SwapMsgReplyState> = Map::new("swap_reply_states");
