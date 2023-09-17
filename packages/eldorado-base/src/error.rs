use cosmwasm_std::StdError;
use thiserror::Error;

pub fn from_std_err(std_error: StdError) -> ContractError {
    ContractError::CustomError {
        val: std_error.to_string(),
    }
}

pub fn to_std_err(contract_error: ContractError) -> StdError {
    StdError::generic_err(contract_error.to_string())
}

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    // common
    #[error("Sender does not have access permissions!")]
    Unauthorized,

    #[error("Undefined Reply ID!")]
    UndefinedReplyId,

    #[error("Event is not found!")]
    EventIsNotFound,

    #[error("Attribute is not found!")]
    AttributeIsNotFound,

    #[error("Wrong MantaSwap message type!")]
    WrongMantaswapMsg,

    #[error("{symbol:?} balance isn't enough!")]
    BalanceIsNotEnough { symbol: String },

    #[error("Recipient parameters are not found!")]
    RecipientParametersAreNotFound,

    #[error("channel_id is not found!")]
    ChannelIdIsNotFound,

    #[error("Wrong IBC parameters: prefix - {prefix:?}, ibc_token - {ibc_token:?}, chain_id - {channel_id:?}!")]
    WrongIbcParameters {
        prefix: String,
        ibc_token: String,
        channel_id: Option<String>,
    },

    #[error("Coin is not found!")]
    CoinIsNotFound,

    #[error("Prefix is not found!")]
    PrefixIsNotFound,

    // eldorado_aggregator_osmosis
    #[error("Pools can not be parsed!")]
    PoolsCanNotBeParsed,

    #[error("Pool is not found!")]
    PoolIsNotFound,
}
