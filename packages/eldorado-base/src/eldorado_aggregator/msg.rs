use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub router_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Called by user to:
    /// 1) swap token on Kujira -> native Kuji
    /// 2) send native Kuji to vault
    SwapIn { vault_address: String },
    /// Called by vault (not restricted) to:
    /// 1) swap native Kuji -> token on Kujira (or don't swap if Kuji is asked asset)
    /// 2) send token on Kujira to user address on Kujira or other Cosmos network
    SwapOut {
        denom_out: String,          // if it's "ukuji" swap isn't required
        user_address: String,       // it can be "kujira1...", "osmo1...", etc.
        channel_id: Option<String>, // must be specified for IBC transfer
    },
    /// Called by admin
    UpdateConfig {
        ibc_timeout_in_mins: Option<u8>,
        router_address: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::eldorado_aggregator::state::Config)]
    QueryConfig {},
}

#[cw_serde]
pub enum MigrateMsg {}
