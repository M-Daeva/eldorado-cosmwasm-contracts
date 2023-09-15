use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Called by user to:
    /// 1) swap token on Osmosis -> native Osmo
    /// 2) send native Osmo to vault
    SwapIn {
        vault_address: String, // must be passed every time to ensure its actuality
    },
    /// Called by vault to:
    /// 1) swap native Osmo -> token on Osmosis
    /// 2) send token on Osmosis to user address on Osmosis or other Cosmos network
    SwapOut {
        user_address: String,
        denom_out: String,
        channel_id: Option<String>, // must be specified to enable IBC transfer
    },
    /// Called by admin
    UpdateConfig { ibc_timeout_in_mins: Option<u8> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::eldorado_aggregator_osmosis::state::Config)]
    QueryConfig {},
}

#[cw_serde]
pub enum MigrateMsg {}
