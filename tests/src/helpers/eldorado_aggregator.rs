use cosmwasm_std::{coin, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use crate::helpers::{
    suite::core::Project,
    suite::types::{ProjectAccount, ProjectCoin, ToAddress},
};

use eldorado_base::{
    eldorado_aggregator_kujira::{
        msg::{ExecuteMsg, QueryMsg},
        state::Config,
    },
    mantaswap,
};

pub trait EldoradoAggregatorExtension {
    fn eldorado_aggregator_try_swap_in<T>(
        &mut self,
        sender: &ProjectAccount,
        vault_address: &ProjectAccount,
        mantaswap_msg: &mantaswap::msg::ExecuteMsg,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone;

    fn eldorado_aggregator_try_swap_out<T>(
        &mut self,
        sender: &ProjectAccount,
        user_address: &ProjectAccount,
        mantaswap_msg: &mantaswap::msg::ExecuteMsg,
        channel_id: &Option<String>,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone;

    fn eldorado_aggregator_try_update_vault<T>(
        &mut self,
        sender: &ProjectAccount,
        vault_address: &ProjectAccount,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone;

    #[allow(clippy::too_many_arguments)]
    fn eldorado_aggregator_try_update_config<T>(
        &mut self,
        sender: &ProjectAccount,
        owner_address: &Option<ProjectAccount>,
        vault_address: &Option<ProjectAccount>,
        ibc_timeout_in_mins: &Option<u8>,
        router_address: &Option<&impl ToString>,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone;

    fn eldorado_aggregator_query_config(&self) -> StdResult<Config>;
}

impl EldoradoAggregatorExtension for Project {
    #[track_caller]
    fn eldorado_aggregator_try_swap_in<T>(
        &mut self,
        sender: &ProjectAccount,
        vault_address: &ProjectAccount,
        mantaswap_msg: &mantaswap::msg::ExecuteMsg,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone,
    {
        let send_funds = &if amount_in.is_some() && denom_in.is_some() {
            vec![coin(
                Uint128::from(amount_in.clone().unwrap()).u128(),
                denom_in.unwrap().to_string(),
            )]
        } else {
            vec![]
        };

        self.app
            .execute_contract(
                sender.to_address(),
                self.get_eldorado_aggregator_address(),
                &ExecuteMsg::SwapIn {
                    vault_address: vault_address.to_string(),
                    mantaswap_msg: mantaswap_msg.to_owned(),
                },
                send_funds,
            )
            .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    fn eldorado_aggregator_try_swap_out<T>(
        &mut self,
        sender: &ProjectAccount,
        user_address: &ProjectAccount,
        mantaswap_msg: &mantaswap::msg::ExecuteMsg,
        channel_id: &Option<String>,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone,
    {
        let send_funds = &if amount_in.is_some() && denom_in.is_some() {
            vec![coin(
                Uint128::from(amount_in.clone().unwrap()).u128(),
                denom_in.unwrap().to_string(),
            )]
        } else {
            vec![]
        };

        self.app
            .execute_contract(
                sender.to_address(),
                self.get_eldorado_aggregator_address(),
                &ExecuteMsg::SwapOut {
                    user_address: user_address.to_string(),
                    mantaswap_msg: mantaswap_msg.to_owned(),
                    channel_id: channel_id.to_owned(),
                },
                send_funds,
            )
            .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    fn eldorado_aggregator_try_update_vault<T>(
        &mut self,
        sender: &ProjectAccount,
        vault_address: &ProjectAccount,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone,
    {
        let send_funds = &if amount_in.is_some() && denom_in.is_some() {
            vec![coin(
                Uint128::from(amount_in.clone().unwrap()).u128(),
                denom_in.unwrap().to_string(),
            )]
        } else {
            vec![]
        };

        self.app
            .execute_contract(
                sender.to_address(),
                self.get_eldorado_aggregator_address(),
                &ExecuteMsg::UpdateVault {
                    vault_address: vault_address.to_string(),
                },
                send_funds,
            )
            .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    fn eldorado_aggregator_try_update_config<T>(
        &mut self,
        sender: &ProjectAccount,
        owner_address: &Option<ProjectAccount>,
        vault_address: &Option<ProjectAccount>,
        ibc_timeout_in_mins: &Option<u8>,
        router_address: &Option<&impl ToString>,
        amount_in: &Option<T>,
        denom_in: &Option<ProjectCoin>,
    ) -> StdResult<AppResponse>
    where
        Uint128: From<T>,
        T: Clone,
    {
        let send_funds = &if amount_in.is_some() && denom_in.is_some() {
            vec![coin(
                Uint128::from(amount_in.clone().unwrap()).u128(),
                denom_in.unwrap().to_string(),
            )]
        } else {
            vec![]
        };

        self.app
            .execute_contract(
                sender.to_address(),
                self.get_eldorado_aggregator_address(),
                &ExecuteMsg::UpdateConfig {
                    owner_address: Some(owner_address.unwrap().to_string()),
                    vault_address: Some(vault_address.unwrap().to_string()),
                    ibc_timeout_in_mins: ibc_timeout_in_mins.to_owned(),
                    router_address: Some(router_address.unwrap().to_string()),
                },
                send_funds,
            )
            .map_err(|err| err.downcast().unwrap())
    }

    #[track_caller]
    fn eldorado_aggregator_query_config(&self) -> StdResult<Config> {
        self.app.wrap().query_wasm_smart(
            self.get_eldorado_aggregator_address(),
            &QueryMsg::QueryConfig {},
        )
    }
}
