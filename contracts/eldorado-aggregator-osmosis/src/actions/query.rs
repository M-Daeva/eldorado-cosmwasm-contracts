use cosmwasm_std::{Deps, Env, StdResult};

use osmosis_std::types::osmosis::{gamm::v1beta1::Pool, poolmanager::v1beta1::PoolmanagerQuerier};

use eldorado_base::{
    eldorado_aggregator_osmosis::state::{Config, CONFIG},
    error::{to_std_err, ContractError},
};

pub fn query_pool(deps: Deps, _env: Env, pool_id: u64) -> StdResult<Pool> {
    PoolmanagerQuerier::new(&deps.querier)
        .pool(pool_id)?
        .pool
        .ok_or(to_std_err(ContractError::PoolIsNotFound))?
        .try_into()
        .map_err(|_| to_std_err(ContractError::PoolsCanNotBeParsed))
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
