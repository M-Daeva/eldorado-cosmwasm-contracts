use cosmwasm_std::{DepsMut, Env, Response};

use eldorado_base::{error::ContractError, mantaswap::msg::MigrateMsg};

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
