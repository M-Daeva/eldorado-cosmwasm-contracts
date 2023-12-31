#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use eldorado_base::{
    eldorado_aggregator_osmosis::{
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
        state::{SWAP_IN_REPLY, SWAP_OUT_REPLY},
    },
    error::ContractError,
};

use crate::actions::{
    execute::{try_swap_in, try_swap_out, try_update_config},
    instantiate::try_instantiate,
    other::{migrate_contract, swap_in_transfer, swap_out_transfer},
    query::query_config,
};

/// Creates a new contract with the specified parameters packed in the "msg" variable
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    try_instantiate(deps, env, info, msg)
}

/// Exposes all the execute functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SwapIn {
            vault_address,
            pool_id,
        } => try_swap_in(deps, env, info, vault_address, pool_id),
        ExecuteMsg::SwapOut {
            user_address,
            pool_id,
            channel_id,
        } => try_swap_out(deps, env, info, user_address, pool_id, channel_id),
        ExecuteMsg::UpdateConfig {
            ibc_timeout_in_mins,
        } => try_update_config(deps, env, info, ibc_timeout_in_mins),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_binary(&query_config(deps, env)?),
    }
}

/// Exposes all the replies available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    let Reply { id, result } = reply;

    match id {
        SWAP_IN_REPLY => swap_in_transfer(deps, env, &result),
        SWAP_OUT_REPLY => swap_out_transfer(deps, env, &result),
        _ => Err(ContractError::UndefinedReplyId),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
