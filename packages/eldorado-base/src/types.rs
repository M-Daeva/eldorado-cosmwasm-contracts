use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct RecipientParameters {
    pub recipient_address: Addr,
    pub channel_id: Option<String>,
    pub denom_out: String,
}
