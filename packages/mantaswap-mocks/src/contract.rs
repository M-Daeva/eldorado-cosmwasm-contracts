#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use eldorado_base::{
    error::ContractError,
    mantaswap::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use crate::actions::{
    execute::{try_swap, try_update_config},
    instantiate::try_instantiate,
    other::migrate_contract,
    query::{query_config, query_swaps_response, query_user_score},
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
        ExecuteMsg::Swap {
            stages,
            recipient,
            min_return,
        } => try_swap(deps, env, info, stages, recipient, min_return),
        ExecuteMsg::UpdateConfig {
            fee,
            owner,
            treasury,
            blend_oracle_contract,
        } => try_update_config(deps, env, info, fee, owner, treasury, blend_oracle_contract),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps, env)?),
        QueryMsg::UserScore { address, week } => {
            to_binary(&query_user_score(deps, env, address, week)?)
        }
        QueryMsg::TotalSwaps { week } => to_binary(&query_swaps_response(deps, env, week)?),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
