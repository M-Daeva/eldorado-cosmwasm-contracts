use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct PairInfo {
    pub pool_id: u64,
    pub asset_in_amount: Uint128,
    pub asset_out_amount: Uint128,
    pub osmo_amount: Uint128,
}
