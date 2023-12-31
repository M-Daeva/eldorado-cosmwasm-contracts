use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::types::RecipientParameters;

pub const TIMEOUT_IN_MINS: u8 = 15;
pub const BASE_DENOM: &str = "uosmo";
pub const BASE_PREFIX: &str = "osmo";

pub const SWAP_IN_REPLY: u64 = 1;
pub const SWAP_OUT_REPLY: u64 = 2;

pub const CONFIG: Item<Config> = Item::new("config");
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub ibc_timeout: u64,
}

impl Config {
    pub fn new(admin: &Addr) -> Self {
        Self {
            admin: admin.to_owned(),
            ibc_timeout: (TIMEOUT_IN_MINS as u64) * 60, // timeout in seconds
        }
    }
}

pub const RECIPIENT_PARAMETERS: Item<Vec<RecipientParameters>> = Item::new("recipient parameters");
