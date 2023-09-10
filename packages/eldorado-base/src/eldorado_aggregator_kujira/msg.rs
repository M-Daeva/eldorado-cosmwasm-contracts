use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::mantaswap;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_address: String,
    pub vault_address: String,
    pub router_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Called by user to:
    /// 1) swap token on Kujira -> native Kuji
    /// 2) send native Kuji to vault
    SwapIn {
        vault_address: String, // must be passed every time to ensure its actuality
        mantaswap_msg: mantaswap::msg::ExecuteMsg,
    },
    /// Called by vault to:
    /// 1) swap native Kuji -> token on Kujira (or don't swap if Kuji is asked asset)
    /// 2) send token on Kujira to user address on Kujira or other Cosmos network
    SwapOut {
        user_address: String,
        mantaswap_msg: mantaswap::msg::ExecuteMsg,
        channel_id: Option<String>, // must be specified to enable IBC transfer
    },
    /// Called by owner
    UpdateVault { vault_address: String },
    /// Called by admin
    UpdateConfig {
        owner_address: Option<String>,
        vault_address: Option<String>,
        ibc_timeout_in_mins: Option<u8>,
        router_address: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::eldorado_aggregator_kujira::state::Config)]
    QueryConfig {},
}

#[cw_serde]
pub enum MigrateMsg {}
