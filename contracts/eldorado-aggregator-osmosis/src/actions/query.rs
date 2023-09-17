use cosmwasm_std::{Deps, Env, StdResult, Uint128};

use osmosis_std::types::osmosis::{
    gamm::v1beta1::Pool,
    poolmanager::v1beta1::{PoolmanagerQuerier, SwapAmountInRoute},
};

use eldorado_base::{
    eldorado_aggregator_osmosis::state::{Config, CONFIG},
    error::{to_std_err, ContractError},
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_pool(deps: Deps, pool_id: u64) -> StdResult<Pool> {
    PoolmanagerQuerier::new(&deps.querier)
        .pool(pool_id)?
        .pool
        .ok_or(to_std_err(ContractError::PoolIsNotFound))?
        .try_into()
        .map_err(|_| to_std_err(ContractError::PoolsCanNotBeParsed))
}

pub fn estimate_swap_exact_amount_in(
    deps: Deps,
    pool_id: u64,
    amount_in: Uint128,
    denom_in: &str,
    routes: &Vec<SwapAmountInRoute>,
) -> StdResult<String> {
    Ok(PoolmanagerQuerier::new(&deps.querier)
        .estimate_swap_exact_amount_in(
            pool_id,
            format!("{}{}", amount_in, denom_in),
            routes.to_owned(),
        )?
        .token_out_amount)
}
