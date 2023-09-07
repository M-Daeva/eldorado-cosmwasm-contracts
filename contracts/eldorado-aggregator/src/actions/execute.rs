use cosmwasm_std::{
    coin, to_binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};

use eldorado_base::error::ContractError;

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes([("action", "try_swap_in")]))
}

pub fn try_swap_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom_out: String,
    user_address: String,
    channel_id: Option<String>,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes([("action", "try_swap_out")]))
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ibc_timeout_in_mins: Option<u8>,
    router_address: Option<String>,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes([("action", "try_update_config")]))
}
