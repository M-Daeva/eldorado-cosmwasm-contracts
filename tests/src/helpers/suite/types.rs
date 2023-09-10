use cosmwasm_std::{Addr, Binary, Decimal, StdResult};
use cw_multi_test::AppResponse;

use anyhow::Error;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use eldorado_base::{converters::str_to_dec, math::P6};

pub const DEFAULT_FUNDS_AMOUNT: u128 = 1; // give each user 1 asset (1 CRD, 1 INJ, etc.)
pub const INCREASED_FUNDS_AMOUNT: u128 = 100 * P6; // give admin such amount of assets to ensure providing 1e6 of assets to each pair

pub const DEFAULT_DECIMALS: u8 = 6;
pub const INCREASED_DECIMALS: u8 = 18;

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectAccount {
    #[strum(serialize = "admin")]
    Admin,
    #[strum(serialize = "owner")]
    Owner,
    #[strum(serialize = "alice")]
    Alice,
    #[strum(serialize = "bob")]
    Bob,
}

impl ProjectAccount {
    pub fn get_initial_funds_amount(&self) -> u128 {
        match self {
            ProjectAccount::Admin => INCREASED_FUNDS_AMOUNT,
            ProjectAccount::Owner => DEFAULT_FUNDS_AMOUNT,
            ProjectAccount::Alice => DEFAULT_FUNDS_AMOUNT,
            ProjectAccount::Bob => DEFAULT_FUNDS_AMOUNT,
        }
    }
}

#[derive(Debug, Clone, Copy, Display, IntoStaticStr, EnumIter)]
pub enum ProjectCoin {
    /// Native token
    #[strum(serialize = "ukuji")]
    Kuji,
    /// Factory token
    #[strum(serialize = "factory/uusk")]
    Usk,
    /// IBC token
    #[strum(serialize = "ibc/ushd")]
    Shd,
}

pub trait GetPrice {
    fn get_price(&self) -> Decimal;
}

impl GetPrice for ProjectAsset {
    fn get_price(&self) -> Decimal {
        match self {
            ProjectAsset::Coin(project_coin) => project_coin.get_price(),
        }
    }
}

impl GetPrice for ProjectCoin {
    fn get_price(&self) -> Decimal {
        match self {
            ProjectCoin::Kuji => str_to_dec("0.5"),
            ProjectCoin::Usk => str_to_dec("1"),
            ProjectCoin::Shd => str_to_dec("0.1"),
        }
    }
}

pub trait GetDecimals {
    fn get_decimals(&self) -> u8;
}

impl GetDecimals for ProjectAsset {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectAsset::Coin(project_coin) => project_coin.get_decimals(),
        }
    }
}

impl GetDecimals for ProjectCoin {
    fn get_decimals(&self) -> u8 {
        match self {
            ProjectCoin::Kuji => DEFAULT_DECIMALS,
            ProjectCoin::Usk => DEFAULT_DECIMALS,
            ProjectCoin::Shd => DEFAULT_DECIMALS,
        }
    }
}

pub trait ToAddress {
    fn to_address(&self) -> Addr;
}

impl ToAddress for ProjectAccount {
    fn to_address(&self) -> Addr {
        Addr::unchecked(self.to_string())
    }
}

#[derive(Debug, Clone, Copy, Display)]
pub enum ProjectAsset {
    Coin(ProjectCoin),
}

pub trait ToProjectAsset {
    fn to_project_asset(&self) -> ProjectAsset;
}

impl ToProjectAsset for ProjectCoin {
    fn to_project_asset(&self) -> ProjectAsset {
        ProjectAsset::Coin(*self)
    }
}

#[derive(Debug)]
pub enum WrappedResponse {
    Execute(Result<AppResponse, Error>),
    Query(StdResult<Binary>),
}

pub trait WrapIntoResponse {
    fn wrap(self) -> WrappedResponse;
}

impl WrapIntoResponse for Result<AppResponse, Error> {
    fn wrap(self) -> WrappedResponse {
        WrappedResponse::Execute(self)
    }
}

impl WrapIntoResponse for StdResult<Binary> {
    fn wrap(self) -> WrappedResponse {
        WrappedResponse::Query(self)
    }
}
