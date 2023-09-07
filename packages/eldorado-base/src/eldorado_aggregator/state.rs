use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const TIMEOUT_IN_MINS: u8 = 15;

pub const CONFIG: Item<Config> = Item::new("config");
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub router: Addr,
    pub ibc_timeout: u64,
}

impl Config {
    pub fn new(admin: &Addr, router: &Addr) -> Self {
        Self {
            admin: admin.to_owned(),
            router: router.to_owned(),
            ibc_timeout: (TIMEOUT_IN_MINS as u64) * 60, // timeout in seconds
        }
    }
}
