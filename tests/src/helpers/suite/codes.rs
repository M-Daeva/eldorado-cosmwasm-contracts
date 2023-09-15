use cosmwasm_std::Addr;
use cw_multi_test::ContractWrapper;

use crate::helpers::suite::core::Project;

pub trait WithCodes {
    // store packages
    fn store_mantaswap_mocks_code(&mut self) -> u64;

    // store contracts
    fn store_eldorado_aggregator_kujira_code(&mut self) -> u64;
    fn store_eldorado_aggregator_osmosis_code(&mut self) -> u64;

    // instantiate packages
    fn instantiate_mantaswap_mocks(&mut self, mantaswap_mocks_code_id: u64) -> Addr;

    // instantiate contracts
    fn instantiate_eldorado_aggregator_kujira(
        &mut self,
        eldorado_aggregator_kujira_code_id: u64,
        router_address: &Addr,
    ) -> Addr;

    fn instantiate_eldorado_aggregator_osmosis(
        &mut self,
        eldorado_aggregator_osmosis_code_id: u64,
    ) -> Addr;
}

impl WithCodes for Project {
    // store packages
    fn store_mantaswap_mocks_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            mantaswap_mocks::contract::execute,
            mantaswap_mocks::contract::instantiate,
            mantaswap_mocks::contract::query,
        )))
    }

    // store contracts
    fn store_eldorado_aggregator_kujira_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                eldorado_aggregator_kujira::contract::execute,
                eldorado_aggregator_kujira::contract::instantiate,
                eldorado_aggregator_kujira::contract::query,
            )
            .with_reply(eldorado_aggregator_kujira::contract::reply),
        ))
    }

    fn store_eldorado_aggregator_osmosis_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                eldorado_aggregator_osmosis::contract::execute,
                eldorado_aggregator_osmosis::contract::instantiate,
                eldorado_aggregator_osmosis::contract::query,
            )
            .with_reply(eldorado_aggregator_osmosis::contract::reply),
        ))
    }

    // instantiate packages
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

    // instantiate contracts
    fn instantiate_eldorado_aggregator_kujira(
        &mut self,
        eldorado_aggregator_kujira_code_id: u64,
        router_address: &Addr,
    ) -> Addr {
        self.instantiate_contract(
            eldorado_aggregator_kujira_code_id,
            "eldorado_aggregator_kujira",
            &eldorado_base::eldorado_aggregator_kujira::msg::InstantiateMsg {
                router_address: router_address.to_string(),
            },
        )
    }

    fn instantiate_eldorado_aggregator_osmosis(
        &mut self,
        eldorado_aggregator_osmosis_code_id: u64,
    ) -> Addr {
        self.instantiate_contract(
            eldorado_aggregator_osmosis_code_id,
            "eldorado_aggregator_osmosis",
            &eldorado_base::eldorado_aggregator_osmosis::msg::InstantiateMsg {},
        )
    }
}
