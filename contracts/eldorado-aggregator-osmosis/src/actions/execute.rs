use cosmwasm_std::{CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::{
        gamm::v1beta1::{MsgSwapExactAmountIn, Pool},
        poolmanager::v1beta1::SwapAmountInRoute,
    },
};

use cw_utils::{must_pay, nonpayable, one_coin};

use eldorado_base::{
    converters::{str_to_dec, u128_to_dec},
    eldorado_aggregator_osmosis::state::{
        Config, CONFIG, DENOM_OSMO, RECIPIENT_PARAMETERS, SWAP_IN_REPLY, SWAP_OUT_REPLY,
    },
    error::ContractError,
    types::RecipientParameters,
};

use crate::actions::query::query_pool;

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
    pool_id: u64,
) -> Result<Response, ContractError> {
    let coin = one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(
        &deps.as_ref(),
        &env,
        coin.amount,
        &coin.denom,
        pool_id,
        &None,
    )?;
    let submsg = SubMsg::reply_on_success(swap_msg, SWAP_IN_REPLY);

    RECIPIENT_PARAMETERS.update(
        deps.storage,
        |mut x| -> Result<Vec<RecipientParameters>, ContractError> {
            x.push(RecipientParameters {
                recipient_address: deps.api.addr_validate(&vault_address)?,
                channel_id: None,
            });

            Ok(x)
        },
    )?;

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
    let amount = must_pay(&info, DENOM_OSMO)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(
        &deps.as_ref(),
        &env,
        amount,
        DENOM_OSMO,
        pool_id,
        &channel_id,
    )?;
    let submsg = SubMsg::reply_on_success(swap_msg, SWAP_OUT_REPLY);

    RECIPIENT_PARAMETERS.update(
        deps.storage,
        |mut x| -> Result<Vec<RecipientParameters>, ContractError> {
            x.push(RecipientParameters {
                recipient_address: deps.api.addr_validate(&user_address)?,
                channel_id,
            });

            Ok(x)
        },
    )?;

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
    deps: &Deps,
    env: &Env,
    amount_in: Uint128,
    denom_in: &str,
    pool_id: u64,
    channel_id: &Option<String>,
) -> Result<CosmosMsg, ContractError> {
    let Pool {
        id, pool_assets, ..
    } = query_pool(deps.to_owned(), env.to_owned(), pool_id)?;

    let asset_list = pool_assets
        .into_iter()
        .map(|x| -> Result<Coin, ContractError> { x.token.ok_or(ContractError::CoinIsNotFound) })
        .collect::<Result<Vec<Coin>, ContractError>>()?;

    let asset_in = asset_list
        .iter()
        .find(|x| x.denom == denom_in)
        .ok_or(ContractError::CoinIsNotFound)?;

    let asset_out = asset_list
        .iter()
        .find(|x| x.denom != denom_in)
        .ok_or(ContractError::CoinIsNotFound)?;

    verify_ibc_parameters(&asset_out.denom, channel_id)?;

    let token_out_min_amount = (str_to_dec("0.9")
        * u128_to_dec(amount_in)
        * (str_to_dec(&asset_in.amount) / str_to_dec(&asset_in.amount)))
    .to_string();

    let routes = vec![SwapAmountInRoute {
        pool_id: id,
        token_out_denom: asset_out.denom.to_owned(),
    }];

    let swap_msg = MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes,
        token_in: Some(Coin {
            denom: denom_in.to_string(),
            amount: amount_in.to_string(),
        }),
        token_out_min_amount,
    };

    Ok(swap_msg.into())
}

fn verify_ibc_parameters(
    ibc_token: &str,
    channel_id: &Option<String>,
) -> Result<(), ContractError> {
    if channel_id.is_some() && !ibc_token.contains("ibc/") {
        Err(ContractError::AssetIsNotIbcToken)?;
    }

    Ok(())
}
