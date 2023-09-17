use std::str::FromStr;

use cosmwasm_std::{
    Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, Response, SubMsgResponse,
    SubMsgResult, Uint128,
};

use osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountInResponse;

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
        denom_out,
    } = recipient_parameters_list
        .get(0)
        .ok_or(ContractError::RecipientParametersAreNotFound)?;

    let recipient_parameters_tail =
        recipient_parameters_list[1..recipient_parameters_list.len()].to_vec();

    RECIPIENT_PARAMETERS.save(deps.storage, &recipient_parameters_tail)?;

    let mut amount_out_string: Option<String> = None;

    if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = result.to_owned() {
        let MsgSwapExactAmountInResponse { token_out_amount } =
            b.try_into().map_err(ContractError::Std)?;

        amount_out_string = Some(token_out_amount);
    }

    let amount_out = Uint128::from_str(&amount_out_string.ok_or(ContractError::CoinIsNotFound)?)?;

    let balance = deps
        .querier
        .query_balance(env.contract.address.as_str(), denom_out)?;

    if balance.amount < amount_out {
        Err(ContractError::BalanceIsNotEnough {
            symbol: denom_out.clone(),
        })?;
    }

    Ok((
        amount_out,
        denom_out.to_string(),
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
