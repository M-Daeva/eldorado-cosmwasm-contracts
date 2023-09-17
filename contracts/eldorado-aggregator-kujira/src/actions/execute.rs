use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg,
};

use cw_utils::{must_pay, nonpayable, one_coin};

use eldorado_base::{
    converters::get_addr_by_prefix,
    eldorado_aggregator_kujira::state::{
        Config, BASE_DENOM, BASE_PREFIX, CHAIN_ID_DEV, CONFIG, RECIPIENT_PARAMETERS, SWAP_IN_REPLY,
        SWAP_OUT_REPLY,
    },
    error::ContractError,
    mantaswap,
    types::RecipientParameters,
};

pub fn try_swap_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    vault_address: String,
    mantaswap_msg: mantaswap::msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    let coin = one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let wasm_msg = get_wasm_msg(
        deps,
        &env,
        &vault_address,
        &mantaswap_msg,
        &vec![coin],
        &None,
    )?;
    let submsg = SubMsg::reply_on_success(CosmosMsg::Wasm(wasm_msg), SWAP_IN_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_swap_in")]))
}

pub fn try_swap_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_address: String,
    mantaswap_msg: mantaswap::msg::ExecuteMsg,
    channel_id: Option<String>,
) -> Result<Response, ContractError> {
    let amount = must_pay(&info, BASE_DENOM)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let coin = Coin {
        denom: BASE_DENOM.to_string(),
        amount,
    };
    let wasm_msg = get_wasm_msg(
        deps,
        &env,
        &user_address,
        &mantaswap_msg,
        &vec![coin],
        &channel_id,
    )?;
    let submsg = SubMsg::reply_on_success(CosmosMsg::Wasm(wasm_msg), SWAP_OUT_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attributes([("action", "try_swap_out")]))
}

pub fn try_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    ibc_timeout_in_mins: Option<u8>,
    router_address: Option<String>,
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

            if let Some(x) = router_address {
                config.router = deps.api.addr_validate(&x)?;
                attrs.push(("router".to_string(), config.router.to_string()));
            }

            Ok(config)
        },
    )?;

    Ok(Response::new().add_attributes(attrs))
}

fn get_wasm_msg(
    deps: DepsMut,
    env: &Env,
    recipient_address: &str,
    mantaswap_msg: &mantaswap::msg::ExecuteMsg,
    funds: &Vec<Coin>,
    channel_id: &Option<String>,
) -> Result<WasmMsg, ContractError> {
    let router = CONFIG.load(deps.storage)?.router;

    let recipient_address = verify_ibc_parameters(
        deps.as_ref(),
        env,
        mantaswap_msg,
        channel_id,
        recipient_address,
    )?;

    let mut min_return_funds: Option<Vec<Coin>> = None;

    let swap_msg = match mantaswap_msg {
        mantaswap::msg::ExecuteMsg::Swap {
            stages, min_return, ..
        } => {
            min_return_funds = min_return.to_owned();

            mantaswap::msg::ExecuteMsg::Swap {
                stages: stages.to_owned(),
                recipient: Some(env.contract.address.clone()),
                min_return: min_return.to_owned(),
            }
        }
        _ => Err(ContractError::WrongMantaswapMsg)?,
    };

    let wasm_msg = WasmMsg::Execute {
        contract_addr: router.to_string(),
        msg: to_binary(&swap_msg)?,
        funds: funds.to_owned(),
    };

    let min_return_funds = min_return_funds.ok_or(ContractError::CoinIsNotFound)?;
    let Coin { denom, .. } = min_return_funds
        .get(0)
        .ok_or(ContractError::CoinIsNotFound)?;

    RECIPIENT_PARAMETERS.update(
        deps.storage,
        |mut x| -> Result<Vec<RecipientParameters>, ContractError> {
            x.push(RecipientParameters {
                recipient_address,
                channel_id: channel_id.to_owned(),
                denom_out: denom.to_string(),
            });

            Ok(x)
        },
    )?;

    Ok(wasm_msg)
}

fn verify_ibc_parameters(
    deps: Deps,
    env: &Env,
    mantaswap_msg: &mantaswap::msg::ExecuteMsg,
    channel_id: &Option<String>,
    recipient_address: &str,
) -> Result<Addr, ContractError> {
    let address_parts = recipient_address.split('1').collect::<Vec<&str>>();
    let prefix = address_parts
        .first()
        .ok_or(ContractError::PrefixIsNotFound)?;

    match mantaswap_msg {
        mantaswap::msg::ExecuteMsg::Swap {
            min_return: Some(coins),
            ..
        } => {
            let Coin { denom, .. } = coins.first().ok_or(ContractError::CoinIsNotFound)?;

            if channel_id.is_some() && !(denom.contains("ibc/") && (prefix != &BASE_PREFIX)) {
                Err(ContractError::WrongIbcParameters {
                    prefix: prefix.to_string(),
                    ibc_token: denom.to_string(),
                    channel_id: channel_id.to_owned(),
                })?;
            }

            let address = if env.block.chain_id == CHAIN_ID_DEV {
                deps.api.addr_validate(recipient_address)?
            } else {
                deps.api
                    .addr_validate(&get_addr_by_prefix(recipient_address, BASE_PREFIX)?)?;

                Addr::unchecked(recipient_address)
            };

            Ok(address)
        }
        _ => Err(ContractError::WrongMantaswapMsg)?,
    }
}
