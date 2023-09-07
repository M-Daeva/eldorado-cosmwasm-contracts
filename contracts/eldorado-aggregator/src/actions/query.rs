use cosmwasm_std::{Coin, Decimal, Deps, Env, StdResult};

use eldorado_base::eldorado_aggregator::state::{Config, CONFIG};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
