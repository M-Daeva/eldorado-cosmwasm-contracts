use cosmwasm_std::{CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, SubMsg, Uint128};

use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::{
        gamm::v1beta1::{MsgSwapExactAmountIn, PoolAsset},
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

use crate::actions::query::query_pools;

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
) -> Result<Response, ContractError> {
    let coin = one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(&deps.as_ref(), &env, coin.amount, &coin.denom, DENOM_OSMO)?;
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
    denom_out: String,
    channel_id: Option<String>,
) -> Result<Response, ContractError> {
    verify_ibc_parameters(&denom_out, &channel_id)?;

    let amount = must_pay(&info, DENOM_OSMO)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let swap_msg = get_swap_msg(&deps.as_ref(), &env, amount, DENOM_OSMO, &denom_out)?;
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
    denom_out: &str,
) -> Result<CosmosMsg, ContractError> {
    let pools = query_pools(deps.to_owned(), env.to_owned())?;
    let pool = pools
        .iter()
        .find(|x| {
            let mut denom_pair_list: Vec<String> = vec![];

            for PoolAsset { token, .. } in &x.pool_assets {
                if let Some(Coin { denom, .. }) = token {
                    denom_pair_list.push(denom.to_string());
                }
            }

            denom_pair_list.contains(&denom_in.to_string())
                && denom_pair_list.contains(&denom_out.to_string())
        })
        .ok_or(ContractError::PoolIsNotFound)?;

    let asset1 = &pool
        .pool_assets
        .get(0)
        .ok_or(ContractError::PoolIsNotFound)?
        .token
        .clone()
        .ok_or(ContractError::CoinIsNotFound)?;

    let asset2 = &pool
        .pool_assets
        .get(1)
        .ok_or(ContractError::PoolIsNotFound)?
        .token
        .clone()
        .ok_or(ContractError::CoinIsNotFound)?;

    let (asset_in, asset_out) = if asset1.denom == denom_in {
        (asset1, asset2)
    } else {
        (asset2, asset1)
    };

    let token_out_min_amount = (str_to_dec("0.9")
        * u128_to_dec(amount_in)
        * (str_to_dec(&asset_out.amount) / str_to_dec(&asset_in.amount)))
    .to_string();

    let routes = vec![SwapAmountInRoute {
        pool_id: pool.id,
        token_out_denom: denom_out.to_string(),
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
    denom_out: &str,
    channel_id: &Option<String>,
) -> Result<(), ContractError> {
    if channel_id.is_some() && !denom_out.contains("ibc/") {
        Err(ContractError::AssetIsNotIbcToken)?;
    }

    Ok(())
}
