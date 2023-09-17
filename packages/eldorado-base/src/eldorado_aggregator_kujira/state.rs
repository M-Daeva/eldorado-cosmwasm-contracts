use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::types::RecipientParameters;

pub const TIMEOUT_IN_MINS: u8 = 15;
pub const BASE_DENOM: &str = "ukuji";
pub const BASE_PREFIX: &str = "kujira";
pub const CHAIN_ID_DEV: &str = "devnet-1";

pub const SWAP_IN_REPLY: u64 = 1;
pub const SWAP_OUT_REPLY: u64 = 2;

pub const CONFIG: Item<Config> = Item::new("config");
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub router: Addr,
    pub ibc_timeout: u64,
    pub chain_id: String,
}

impl Config {
    pub fn new(admin: &Addr, router: &Addr, chain_id: &str) -> Self {
        Self {
            admin: admin.to_owned(),
            router: router.to_owned(),
            ibc_timeout: (TIMEOUT_IN_MINS as u64) * 60, // timeout in seconds
            chain_id: chain_id.to_string(),
        }
    }
}

pub const RECIPIENT_PARAMETERS: Item<Vec<RecipientParameters>> = Item::new("recipient parameters");
