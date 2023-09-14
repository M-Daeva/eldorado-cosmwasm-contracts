use std::str::FromStr;

use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, Event, IbcMsg, IbcTimeout, Response,
    SubMsgResult, Uint128,
};

use eldorado_base::{
    eldorado_aggregator_osmosis::{
        msg::MigrateMsg,
        state::{CONFIG, RECIPIENT_PARAMETERS},
    },
    error::ContractError,
    types::RecipientParameters,
};

pub fn swap_in_transfer(
    mut deps: DepsMut,
    env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    let (amount, denom, recipient_address, ..) = parse_attributes(&mut deps, &env, result)?;

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient_address.to_string(),
        amount: vec![Coin {
            amount,
            denom: denom.clone(),
        }],
    });

    Ok(Response::new().add_message(msg).add_attributes([(
        "digest",
        format!("{} {} {}", amount, denom, recipient_address),
    )]))
}

pub fn swap_out_transfer(
    mut deps: DepsMut,
    env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    let (amount, denom, recipient_address, channel_id) = parse_attributes(&mut deps, &env, result)?;

    // choose the type of transfer
    let msg = match channel_id {
        None => CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient_address.to_string(),
            amount: vec![Coin {
                amount,
                denom: denom.clone(),
            }],
        }),
        Some(channel_id) => {
            let timestamp = env
                .block
                .time
                .plus_seconds(CONFIG.load(deps.storage)?.ibc_timeout);

            CosmosMsg::Ibc(IbcMsg::Transfer {
                channel_id,
                to_address: recipient_address.to_string(),
                amount: Coin {
                    amount,
                    denom: denom.clone(),
                },
                timeout: IbcTimeout::with_timestamp(timestamp),
            })
        }
    };

    Ok(Response::new().add_message(msg).add_attributes([(
        "digest",
        format!("{} {} {}", amount, denom, recipient_address),
    )]))
}

fn parse_attributes(
    deps: &mut DepsMut,
    env: &Env,
    result: &SubMsgResult,
) -> Result<(Uint128, String, Addr, Option<String>), ContractError> {
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

    let res = result
        .to_owned()
        .into_result()
        .map_err(|e| ContractError::CustomError { val: e })?;

    let mut transfer_events: Vec<Event> = vec![];

    for event in res.events {
        if event.ty.contains("transfer") {
            for attr in &event.attributes {
                if (attr.key == "recipient") && (attr.value == env.contract.address.as_ref()) {
                    transfer_events.push(event.clone());
                    break;
                }
            }
        }
    }

    let event = transfer_events
        .last()
        .ok_or(ContractError::EventIsNotFound)?;

    let coin_string = &event
        .attributes
        .iter()
        .find(|x| x.key == "amount")
        .ok_or(ContractError::AttributeIsNotFound)?
        .value;

    let Coin { denom, amount } = Coin::from_str(coin_string)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    let balance = deps
        .querier
        .query_balance(env.contract.address.as_str(), &denom)?;

    if balance.amount < amount {
        Err(ContractError::BalanceIsNotEnough {
            symbol: denom.clone(),
        })?;
    }

    Ok((
        amount,
        denom,
        recipient_address.to_owned(),
        channel_id.to_owned(),
    ))
}

pub fn migrate_contract(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new())
}
