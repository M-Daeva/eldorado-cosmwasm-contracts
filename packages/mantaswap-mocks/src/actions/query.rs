use cosmwasm_std::{Addr, Deps, Env, StdResult};

use eldorado_base::mantaswap::msg::{ConfigResponse, SwapsResponse, UserResponse};

pub fn query_config(_deps: Deps, _env: Env) -> StdResult<ConfigResponse> {
    let addr = &Addr::unchecked("contract42");

    Ok(ConfigResponse {
        owner: addr.to_owned(),
        fee: 42,
        treasury: addr.to_owned(),
        blend_oracle_contract: addr.to_owned(),
    })
}

pub fn query_user_score(
    _deps: Deps,
    _env: Env,
    _address: String,
    _week: u128,
) -> StdResult<UserResponse> {
    Ok(UserResponse {
        address: "contract42".to_string(),
        week: 42,
        value: 42,
    })
}

pub fn query_swaps_response(_deps: Deps, _env: Env, _week: u128) -> StdResult<SwapsResponse> {
    Ok(SwapsResponse {
        week: 42,
        value: 42,
    })
}
