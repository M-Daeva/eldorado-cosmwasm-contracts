use cosmwasm_std::{
    to_binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg,
};

use cw_utils::{must_pay, nonpayable, one_coin};

use eldorado_base::{
    eldorado_aggregator_kujira::{
        state::{Config, CONFIG, DENOM_KUJI, RECIPIENT_PARAMETERS, SWAP_IN_REPLY, SWAP_OUT_REPLY},
        types::RecipientParameters,
    },
    error::ContractError,
    mantaswap,
};

pub fn try_swap_in(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    vault_address: String,
    mantaswap_msg: mantaswap::msg::ExecuteMsg,
) -> Result<Response, ContractError> {
    let coin = one_coin(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let wasm_msg = get_wasm_msg(&deps.as_ref(), &vault_address, &mantaswap_msg, &vec![coin])?;
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
    if info.sender != CONFIG.load(deps.storage)?.vault {
        Err(ContractError::Unauthorized)?;
    }

    verify_ibc_parameters(&mantaswap_msg, &channel_id)?;

    let amount = must_pay(&info, DENOM_KUJI)
        .map_err(|e| ContractError::CustomError { val: e.to_string() })?;
    let coin = Coin {
        denom: DENOM_KUJI.to_string(),
        amount,
    };
    let wasm_msg = get_wasm_msg(
        &deps.as_ref(),
        env.contract.address.as_str(),
        &mantaswap_msg,
        &vec![coin],
    )?;
    let submsg = SubMsg::reply_on_success(CosmosMsg::Wasm(wasm_msg), SWAP_OUT_REPLY);

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

pub fn try_update_vault(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    vault_address: String,
) -> Result<Response, ContractError> {
    let config = &CONFIG.load(deps.storage)?;

    if (info.sender != config.admin) && (info.sender != config.owner) {
        Err(ContractError::Unauthorized)?;
    }

    nonpayable(&info).map_err(|e| ContractError::CustomError { val: e.to_string() })?;

    CONFIG.save(
        deps.storage,
        &Config {
            vault: deps.api.addr_validate(&vault_address)?,
            ..config.to_owned()
        },
    )?;

    Ok(Response::new().add_attributes([("action", "try_update_vault"), ("vault", &vault_address)]))
}

pub fn try_update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner_address: Option<String>,
    vault_address: Option<String>,
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

            if let Some(x) = owner_address {
                config.owner = deps.api.addr_validate(&x)?;
                attrs.push(("owner".to_string(), config.owner.to_string()));
            }

            if let Some(x) = vault_address {
                config.vault = deps.api.addr_validate(&x)?;
                attrs.push(("vault".to_string(), config.vault.to_string()));
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
    deps: &Deps,
    recipient_address: &str,
    mantaswap_msg: &mantaswap::msg::ExecuteMsg,
    funds: &Vec<Coin>,
) -> Result<WasmMsg, ContractError> {
    let recipient = deps.api.addr_validate(recipient_address)?;
    let router = CONFIG.load(deps.storage)?.router;

    let swap_msg = match mantaswap_msg {
        mantaswap::msg::ExecuteMsg::Swap {
            stages, min_return, ..
        } => mantaswap::msg::ExecuteMsg::Swap {
            stages: stages.to_owned(),
            recipient: Some(recipient),
            min_return: min_return.to_owned(),
        },
        _ => Err(ContractError::WrongMantaswapMsg)?,
    };

    let wasm_msg = WasmMsg::Execute {
        contract_addr: router.to_string(),
        msg: to_binary(&swap_msg)?,
        funds: funds.to_owned(),
    };

    Ok(wasm_msg)
}

fn verify_ibc_parameters(
    mantaswap_msg: &mantaswap::msg::ExecuteMsg,
    channel_id: &Option<String>,
) -> Result<(), ContractError> {
    match mantaswap_msg {
        mantaswap::msg::ExecuteMsg::Swap {
            min_return: Some(coins),
            ..
        } => {
            let Coin { denom, .. } = coins.get(0).ok_or(ContractError::CoinIsNotFound)?;

            if channel_id.is_some() && !denom.contains("ibc/") {
                Err(ContractError::AssetIsNotIbcToken)?;
            }

            Ok(())
        }
        _ => Err(ContractError::WrongMantaswapMsg)?,
    }
}
