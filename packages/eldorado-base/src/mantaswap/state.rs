use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub fee: u128,
    pub treasury: Addr,
    pub blend_oracle_contract: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const USERS: Map<(&Addr, u128), u128> = Map::new("users");
pub const SWAPS: Map<u128, u128> = Map::new("swaps");
