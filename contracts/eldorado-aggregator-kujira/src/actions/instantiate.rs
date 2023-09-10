use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use eldorado_base::{
    eldorado_aggregator_kujira::{
        msg::InstantiateMsg,
        state::{Config, CONFIG, RECIPIENT_PARAMETERS},
    },
    error::ContractError,
};

const CONTRACT_NAME: &str = "crates.io:eldorado_aggregator_kujira";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin = &info.sender;
    let owner = &deps.api.addr_validate(&msg.owner_address)?;
    let vault = &deps.api.addr_validate(&msg.vault_address)?;
    let router = &deps.api.addr_validate(&msg.router_address)?;

    CONFIG.save(deps.storage, &Config::new(admin, owner, vault, router))?;

    RECIPIENT_PARAMETERS.save(deps.storage, &vec![])?;

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attributes([
        ("action", "try_instantiate"),
        ("admin", admin.as_str()),
        ("owner", owner.as_str()),
        ("vault", vault.as_str()),
        ("router", router.as_str()),
    ]))
}
