use cosmwasm_std::{Decimal, Decimal256, StdError, StdResult, Uint128};

use std::str::FromStr;

use bech32::{decode, encode, Variant};

pub fn str_to_dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

pub fn u128_to_dec<T>(num: T) -> Decimal
where
    Uint128: From<T>,
{
    Decimal::from_ratio(Uint128::from(num), Uint128::one())
}

pub fn dec_to_u128(dec: Decimal) -> u128 {
    dec.to_uint_ceil().u128()
}

pub fn dec_to_uint128(dec: Decimal) -> Uint128 {
    dec.to_uint_ceil()
}

pub fn u128_to_dec256<T>(num: T) -> Decimal256
where
    Uint128: From<T>,
{
    Decimal256::from_ratio(Uint128::from(num), Uint128::one())
}

pub fn dec_to_dec256(dec: Decimal) -> Decimal256 {
    Decimal256::from_str(&dec.to_string()).unwrap()
}

pub fn dec256_to_dec(dec256: Decimal256) -> Decimal {
    str_to_dec(
        &dec256
            .to_string()
            .chars()
            .take(Decimal::DECIMAL_PLACES as usize)
            .collect::<String>(),
    )
}

pub fn dec256_to_u128(dec256: Decimal256) -> u128 {
    Uint128::try_from(dec256.to_uint_ceil()).unwrap().u128()
}

pub fn str_vec_to_dec_vec(str_vec: &[&str]) -> Vec<Decimal> {
    str_vec.iter().map(|&x| str_to_dec(x)).collect()
}

pub fn u128_vec_to_uint128_vec(u128_vec: &[u128]) -> Vec<Uint128> {
    u128_vec
        .iter()
        .map(|&x| Uint128::from(x))
        .collect::<Vec<Uint128>>()
}

pub fn get_addr_by_prefix(address: &str, prefix: &str) -> StdResult<String> {
    let (_hrp, data, _) = decode(address).map_err(|e| StdError::generic_err(e.to_string()))?;
    let new_address =
        encode(prefix, data, Variant::Bech32).map_err(|e| StdError::generic_err(e.to_string()))?;
    Ok(new_address)
}
