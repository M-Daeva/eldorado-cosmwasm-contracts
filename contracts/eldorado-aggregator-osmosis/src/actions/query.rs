use cosmwasm_std::{Deps, Env, StdResult};

use eldorado_base::eldorado_aggregator_osmosis::state::{Config, CONFIG};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
