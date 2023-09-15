use cosmwasm_std::{Deps, Env, StdResult};

use osmosis_std::types::osmosis::gamm::v1beta1::{GammQuerier, Pool};

use eldorado_base::{
    eldorado_aggregator_osmosis::state::{Config, CONFIG},
    error::{to_std_err, ContractError},
};

pub fn query_pools(deps: Deps, _env: Env) -> StdResult<Vec<Pool>> {
    GammQuerier::new(&deps.querier)
        .pools(None)?
        .pools
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<Vec<Pool>, _>>()
        .map_err(|_| to_std_err(ContractError::PoolsCanNotBeParsed))
}

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
