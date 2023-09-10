use cosmwasm_std::Addr;
use cw_multi_test::ContractWrapper;

use crate::helpers::suite::{core::Project, types::ProjectAccount};

pub trait WithCodes {
    // store
    // packages
    fn store_mantaswap_mocks_code(&mut self) -> u64;

    // contracts
    fn store_eldorado_aggregator_code(&mut self) -> u64;

    // instantiate
    // packages
    fn instantiate_mantaswap_mocks(&mut self, mantaswap_mocks_code_id: u64) -> Addr;

    // contracts
    fn instantiate_eldorado_aggregator(
        &mut self,
        eldorado_aggregator_code_id: u64,
        owner_address: &ProjectAccount,
        vault_address: &ProjectAccount,
        router_address: &Addr,
    ) -> Addr;
}

impl WithCodes for Project {
    fn store_mantaswap_mocks_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            mantaswap_mocks::contract::execute,
            mantaswap_mocks::contract::instantiate,
            mantaswap_mocks::contract::query,
        )))
    }

    fn store_eldorado_aggregator_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                eldorado_aggregator_kujira::contract::execute,
                eldorado_aggregator_kujira::contract::instantiate,
                eldorado_aggregator_kujira::contract::query,
            )
            .with_reply(eldorado_aggregator_kujira::contract::reply),
        ))
    }

    fn instantiate_mantaswap_mocks(&mut self, mantaswap_mocks_code_id: u64) -> Addr {
        let addr = "contract42";

        self.instantiate_contract(
            mantaswap_mocks_code_id,
            "mantaswap_mocks",
            &eldorado_base::mantaswap::msg::InstantiateMsg {
                blend_oracle_contract: addr.to_string(),
                owner: addr.to_string(),
                treasury: addr.to_string(),
                fee: 42,
            },
        )
    }

    fn instantiate_eldorado_aggregator(
        &mut self,
        eldorado_aggregator_code_id: u64,
        owner_address: &ProjectAccount,
        vault_address: &ProjectAccount,
        router_address: &Addr,
    ) -> Addr {
        self.instantiate_contract(
            eldorado_aggregator_code_id,
            "eldorado_aggregator",
            &eldorado_base::eldorado_aggregator_kujira::msg::InstantiateMsg {
                owner_address: owner_address.to_string(),
                router_address: router_address.to_string(),
                vault_address: vault_address.to_string(),
            },
        )
    }
}
