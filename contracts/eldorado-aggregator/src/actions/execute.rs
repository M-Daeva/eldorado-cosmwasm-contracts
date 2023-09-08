use cosmwasm_std::{
    coin, to_binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};

use cw_utils::{must_pay, nonpayable, one_coin};

use eldorado_base::{
    eldorado_aggregator::state::{Config, CONFIG, DENOM_KUJI},
    error::ContractError,
    mantaswap,
};

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
    mantaswap_msg: mantaswap::msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    let Coin { denom, amount } =
        one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    Ok(Response::new().add_attributes([("action", "try_swap_in")]))
}

pub fn try_swap_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_address: String,
    mantaswap_msg: mantaswap::msg::ExecuteMsg,
    channel_id: Option<String>,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, DENOM_KUJI)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    Ok(Response::new().add_attributes([("action", "try_swap_out")]))
}

pub fn try_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ibc_timeout_in_mins: Option<u8>,
    router_address: Option<String>,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    let mut attrs = vec![("action".to_string(), "try_update_config".to_string())];

    CONFIG.update(
        deps.storage,
        |mut config| -> Result<Config, ContractError> {
            if info.sender != config.admin {
                Err(ContractError::Unauthorized {})?;
            }

            if let Some(x) = ibc_timeout_in_mins {
                config.ibc_timeout = (x as u64) * 60;
                attrs.push(("ibc_timeout".to_string(), config.ibc_timeout.to_string()));
            }

            if let Some(x) = router_address {
                config.router = deps.api.addr_validate(&x)?;
                attrs.push(("router".to_string(), config.router.to_string()));
            }

            Ok(config)
        },
    )?;

    Ok(Response::new().add_attributes(attrs))
}
