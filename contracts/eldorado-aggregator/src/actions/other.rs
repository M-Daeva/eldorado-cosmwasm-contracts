use std::str::FromStr;

use cosmwasm_std::{BalanceResponse, Coin, DepsMut, Env, Response, SubMsgResult};

use eldorado_base::{eldorado_aggregator::msg::MigrateMsg, error::ContractError};

pub fn try_transfer(
    deps: DepsMut,
    env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    // parse abstract account address
    let res = result
        .to_owned()
        .into_result()
        .map_err(|e| ContractError::CustomError { val: e })?;

    let event = res
        .events
        .iter()
        .find(|x| x.ty == "coin_received")
        .ok_or(ContractError::EventIsNotFound)?;

    // get "25698ibc/590CE97A3681BC2058FED1F69B613040209DF3F17B7BD31DFFB8671C4D2CD99B"
    let coin_string = &event
        .attributes
        .get(event.attributes.len() - 2)
        .ok_or(ContractError::AttributeIsNotFound)?
        .value;

    let Coin { denom, amount } = Coin::from_str(&coin_string)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    let balance = deps.querier.query_balance(env.contract.address, &denom)?;

    // TODO: check balance

    // // save to storage
    // let msg = flexifi_base::abstract_account::msg::QueryMsg::QueryConfig {};
    // let config: flexifi_base::abstract_account::state::Config =
    //     deps.querier.query_wasm_smart(account_address, &msg)?;

    // ACCOUNTS.save(deps.storage, &config.owner, account_address)?;

    Ok(Response::default().add_attributes([("action", "try_transfer")]))
}

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
