use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use eldorado_base::{
    eldorado_aggregator::{
        msg::InstantiateMsg,
        state::{Config, CONFIG},
    },
    error::ContractError,
};

const CONTRACT_NAME: &str = "crates.io:eldorado_aggregator";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = &info.sender;
    let router = &deps.api.addr_validate(&msg.router_address)?;

    CONFIG.save(deps.storage, &Config::new(admin, router))?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attributes([
        ("action", "try_instantiate"),
        ("admin", admin.as_str()),
        ("router", router.as_str()),
    ]))
}
