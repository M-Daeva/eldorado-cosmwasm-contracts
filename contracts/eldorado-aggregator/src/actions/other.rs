use cosmwasm_std::{DepsMut, Env, Response};

use eldorado_base::{eldorado_aggregator::msg::MigrateMsg, error::ContractError};

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
