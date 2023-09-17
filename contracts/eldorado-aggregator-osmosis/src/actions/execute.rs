use cosmwasm_std::{Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::{
        gamm::v1beta1::{MsgSwapExactAmountIn, Pool},
        poolmanager::v1beta1::SwapAmountInRoute,
    },
};

use cw_utils::{must_pay, nonpayable, one_coin};

use eldorado_base::{
    converters::get_addr_by_prefix,
    eldorado_aggregator_osmosis::state::{
        Config, BASE_DENOM, BASE_PREFIX, CONFIG, RECIPIENT_PARAMETERS, SWAP_IN_REPLY,
        SWAP_OUT_REPLY,
    },
    error::ContractError,
    types::RecipientParameters,
};

use crate::actions::query::{estimate_swap_exact_amount_in, query_pool};

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
    pool_id: u64,
) -> Result<Response, ContractError> {
    let coin = one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(
        deps,
        &env,
        &vault_address,
        coin.amount,
        &coin.denom,
        pool_id,
        &None,
    )?;
    let submsg = SubMsg::reply_on_success(swap_msg, SWAP_IN_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_swap_in")]))
}

pub fn try_swap_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_address: String,
    pool_id: u64,
    channel_id: Option<String>,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, BASE_DENOM)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(
        deps,
        &env,
        &user_address,
        amount,
        BASE_DENOM,
        pool_id,
        &channel_id,
    )?;
    let submsg = SubMsg::reply_on_success(swap_msg, SWAP_OUT_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_swap_out")]))
}

pub fn try_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ibc_timeout_in_mins: Option<u8>,
) -> Result<Response, ContractError> {
    nonpayable(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    let mut attrs = vec![("action".to_string(), "try_update_config".to_string())];

    CONFIG.update(
        deps.storage,
        |mut config| -> Result<Config, ContractError> {
            if info.sender != config.admin {
                Err(ContractError::Unauthorized {})?;
            }

            if let Some(x) = ibc_timeout_in_mins {
                config.ibc_timeout = (x as u64) * 60;
                attrs.push(("ibc_timeout".to_string(), config.ibc_timeout.to_string()));
            }

            Ok(config)
        },
    )?;

    Ok(Response::new().add_attributes(attrs))
}

fn get_swap_msg(
    deps: DepsMut,
    env: &Env,
    recipient_address: &str,
    amount_in: Uint128,
    denom_in: &str,
    pool_id: u64,
    channel_id: &Option<String>,
) -> Result<CosmosMsg, ContractError> {
    let Pool {
        id, pool_assets, ..
    } = query_pool(deps.as_ref(), pool_id)?;

    let asset_list = pool_assets
        .into_iter()
        .map(|x| -> Result<Coin, ContractError> { x.token.ok_or(ContractError::CoinIsNotFound) })
        .collect::<Result<Vec<Coin>, ContractError>>()?;

    let asset_out = asset_list
        .iter()
        .find(|x| x.denom != denom_in)
        .ok_or(ContractError::CoinIsNotFound)?;
    let denom_out = &asset_out.denom;

    let recipient_address =
        verify_ibc_parameters(deps.as_ref(), denom_out, channel_id, recipient_address)?;

    let routes = vec![SwapAmountInRoute {
        pool_id: id,
        token_out_denom: denom_out.to_string(),
    }];

    let token_out_min_amount =
        estimate_swap_exact_amount_in(deps.as_ref(), id, amount_in, denom_in, &routes)?;

    let swap_msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes,
        token_in: Some(Coin {
            denom: denom_in.to_string(),
            amount: amount_in.to_string(),
        }),
        token_out_min_amount,
    };

    RECIPIENT_PARAMETERS.update(
        deps.storage,
        |mut x| -> Result<Vec<RecipientParameters>, ContractError> {
            x.push(RecipientParameters {
                recipient_address,
                channel_id: channel_id.to_owned(),
                denom_out: denom_out.to_string(),
            });

            Ok(x)
        },
    )?;

    Ok(swap_msg.into())
}

fn verify_ibc_parameters(
    deps: Deps,
    ibc_token: &str,
    channel_id: &Option<String>,
    recipient_address: &str,
) -> Result<Addr, ContractError> {
    let address_parts = recipient_address.split('1').collect::<Vec<&str>>();
    let prefix = address_parts
        .first()
        .ok_or(ContractError::PrefixIsNotFound)?;

    if channel_id.is_some() && (!ibc_token.contains("ibc/") || (prefix == &BASE_PREFIX)) {
        Err(ContractError::WrongIbcParameters {
            prefix: prefix.to_string(),
            ibc_token: ibc_token.to_string(),
            channel_id: channel_id.to_owned(),
        })?;
    }

    deps.api
        .addr_validate(&get_addr_by_prefix(recipient_address, BASE_PREFIX)?)?;

    Ok(Addr::unchecked(recipient_address))
}
