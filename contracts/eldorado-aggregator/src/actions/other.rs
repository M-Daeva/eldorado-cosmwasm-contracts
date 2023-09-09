use std::str::FromStr;

use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, Response, SubMsgResult, Uint128,
};

use eldorado_base::{
    eldorado_aggregator::{
        msg::MigrateMsg,
        state::{CONFIG, RECIPIENT_PARAMETERS},
        types::RecipientParameters,
    },
    error::ContractError,
};

pub fn add_attributes(
    _deps: DepsMut,
    _env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    // add events from Swap response to SwapIn response
    let (recipient, amount, denom) = parse_recipient_amount_denom(result)?;

    Ok(Response::new().add_attributes([
        ("recipient", recipient),
        ("amount", amount.to_string()),
        ("denom", denom),
    ]))
}

pub fn try_transfer(
    deps: DepsMut,
    env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    // parse received amount from Swap response events
    let (_recipient, amount, denom) = parse_recipient_amount_denom(result)?;
    let balance = deps.querier.query_balance(env.contract.address, &denom)?;

    if balance.amount < amount {
        Err(ContractError::BalanceIsNotEnough {
            symbol: denom.clone(),
        })?;
    }

    let recipient_parameters_list = RECIPIENT_PARAMETERS.load(deps.storage)?;

    let RecipientParameters {
        recipient_address,
        channel_id,
    } = recipient_parameters_list
        .get(0)
        .ok_or(ContractError::RecipientParametersAreNotFound)?;

    let recipient_parameters_tail =
        recipient_parameters_list[1..recipient_parameters_list.len()].to_vec();

    RECIPIENT_PARAMETERS.save(deps.storage, &recipient_parameters_tail)?;

    let attributes = [
        ("recipient", recipient_address.to_string()),
        ("amount", amount.to_string()),
        ("denom", denom.clone()),
    ];

    // choose the type of transfer
    let msg = match channel_id.to_owned() {
        None => CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient_address.to_string(),
            amount: vec![Coin { amount, denom }],
        }),
        Some(channel_id) => {
            let timestamp = env
                .block
                .time
                .plus_seconds(CONFIG.load(deps.storage)?.ibc_timeout);

            CosmosMsg::Ibc(IbcMsg::Transfer {
                channel_id,
                to_address: recipient_address.to_string(),
                amount: Coin { amount, denom },
                timeout: IbcTimeout::with_timestamp(timestamp),
            })
        }
    };

    Ok(Response::new().add_message(msg).add_attributes(attributes))
}

fn parse_recipient_amount_denom(
    result: &SubMsgResult,
) -> Result<(String, Uint128, String), ContractError> {
    let res = result
        .to_owned()
        .into_result()
        .map_err(|e| ContractError::CustomError { val: e })?;

    let event = res
        .events
        .iter()
        .find(|x| x.ty == "coin_received")
        .ok_or(ContractError::EventIsNotFound)?;

    let recipient = &event
        .attributes
        .last()
        .ok_or(ContractError::AttributeIsNotFound)?
        .value;

    let coin_string = &event
        .attributes
        .get(event.attributes.len() - 2)
        .ok_or(ContractError::AttributeIsNotFound)?
        .value;

    let Coin { denom, amount } = Coin::from_str(coin_string)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    Ok((recipient.to_string(), amount, denom))
}

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
