use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response};

use kujira::Denom;

use eldorado_base::error::ContractError;

pub fn try_swap(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _stages: Vec<Vec<(Addr, Denom)>>,
    recipient: Option<Addr>,
    min_return: Option<Vec<Coin>>,
) -> Result<Response, ContractError> {
    let recipient = &recipient.map_or(info.sender, |x| x).to_string();
    let coins = &min_return.unwrap();

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_owned(),
        amount: coins.to_owned(),
    });

    let event_first = Event::new("coin_received")
        .add_attribute(
            "amount",
            "25ibc/590CE97A3681BC2058FED1F69B613040209DF3F17B7BD31DFFB8671C4D2CD99B",
        )
        .add_attribute(
            "receiver",
            "kujira15e682nq9jees29rm9j3h030af86lq2qtlejgphlspzqcvs9whf2q00nua5",
        )
        .add_attribute("amount", coins[0].to_string())
        .add_attribute("receiver", recipient);

    let event_last = Event::new("trade")
        .add_attribute("quote_amount", "2528")
        .add_attribute("type", "buy");

    Ok(Response::new()
        .add_message(msg)
        .add_events(vec![event_first, event_last]))
}

pub fn try_update_config(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _fee: Option<u128>,
    _owner: Option<String>,
    _treasury: Option<String>,
    _blend_oracle_contract: Option<String>,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes([("action", "update_config")]))
}
