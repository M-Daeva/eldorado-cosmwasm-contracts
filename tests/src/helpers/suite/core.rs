use cosmwasm_std::{coin, Addr, BankMsg, Coin, CosmosMsg, Timestamp};
use cw_multi_test::{App, AppResponse, Executor};

use serde::Serialize;
use strum::IntoEnumIterator;

use crate::helpers::suite::{
    codes::WithCodes,
    types::{GetDecimals, ProjectAccount, ProjectCoin, ToAddress, WrappedResponse},
};

#[allow(dead_code)]
pub struct Project {
    pub app: App,
    pub logs: WrappedResponse,
    contract_counter: u16,

    // package code id
    mantaswap_mocks_code_id: u64,

    // contract code id
    eldorado_aggregator_code_id: u64,

    // package address
    mantaswap_router_address: Addr,

    // contract address
    eldorado_aggregator_address: Addr,
}

impl Project {
    pub fn create_project_with_balances() -> Self {
        Self {
            app: Self::create_app_with_balances(),
            logs: WrappedResponse::Execute(Ok(AppResponse::default())),
            contract_counter: 0,

            mantaswap_mocks_code_id: 0,

            eldorado_aggregator_code_id: 0,

            mantaswap_router_address: Addr::unchecked(""),

            eldorado_aggregator_address: Addr::unchecked(""),
        }
    }

    pub fn new() -> Self {
        // create app and distribute coins to accounts
        let mut project = Self::create_project_with_balances();

        // register contracts code
        // packages
        let mantaswap_mocks_code_id = project.store_mantaswap_mocks_code();

        // contracts
        let eldorado_aggregator_code_id = project.store_eldorado_aggregator_code();

        let mantaswap_router_address = project.instantiate_mantaswap_mocks(mantaswap_mocks_code_id);

        // instantiate contracts
        let eldorado_aggregator_address = project.instantiate_eldorado_aggregator(
            eldorado_aggregator_code_id,
            &ProjectAccount::Owner,
            &ProjectAccount::Bob,
            &mantaswap_router_address,
        );

        // add funds to mantaswap router
        for project_coin in ProjectCoin::iter() {
            let amount = ProjectAccount::Admin.get_initial_funds_amount()
                * 10u128.pow(project_coin.get_decimals() as u32);

            project
                .app
                .execute(
                    ProjectAccount::Admin.to_address(),
                    CosmosMsg::Bank(BankMsg::Send {
                        to_address: mantaswap_router_address.to_string(),
                        amount: vec![coin(amount / 10, project_coin.to_string())],
                    }),
                )
                .unwrap();
        }

        Self {
            mantaswap_mocks_code_id,

            eldorado_aggregator_code_id,

            mantaswap_router_address,

            eldorado_aggregator_address,

            ..project
        }
    }

    // code id getters
    pub fn get_eldorado_aggregator_code_id(&self) -> u64 {
        self.eldorado_aggregator_code_id
    }

    pub fn get_mantaswap_router_address(&self) -> Addr {
        self.mantaswap_router_address.clone()
    }

    pub fn get_eldorado_aggregator_address(&self) -> Addr {
        self.eldorado_aggregator_address.clone()
    }

    // utils
    pub fn increase_contract_counter(&mut self, step: u16) {
        self.contract_counter += step;
    }

    pub fn get_last_contract_address(&self) -> String {
        format!("contract{}", self.contract_counter)
    }

    pub fn get_timestamp(&self) -> Timestamp {
        self.app.block_info().time
    }

    pub fn wait(&mut self, delay_ns: u64) {
        self.app.update_block(|block| {
            block.time = block.time.plus_nanos(delay_ns);
            block.height += delay_ns / 5_000_000_000;
        });
    }

    pub fn instantiate_contract(
        &mut self,
        code_id: u64,
        label: &str,
        init_msg: &impl Serialize,
    ) -> Addr {
        self.increase_contract_counter(1);

        self.app
            .instantiate_contract(
                code_id,
                ProjectAccount::Admin.to_address(),
                init_msg,
                &[],
                label,
                Some(ProjectAccount::Admin.to_string()),
            )
            .unwrap()
    }

    fn create_app_with_balances() -> App {
        App::new(|router, _api, storage| {
            for project_account in ProjectAccount::iter() {
                let funds: Vec<Coin> = ProjectCoin::iter()
                    .map(|project_coin| {
                        let amount = project_account.get_initial_funds_amount()
                            * 10u128.pow(project_coin.get_decimals() as u32);

                        coin(amount, project_coin.to_string())
                    })
                    .collect();

                router
                    .bank
                    .init_balance(storage, &project_account.to_address(), funds)
                    .unwrap();
            }
        })
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
