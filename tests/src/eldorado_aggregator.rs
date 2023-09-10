use cosmwasm_std::{coin, Addr};

use speculoos::assert_that;

use kujira::Denom;

use eldorado_base::{eldorado_aggregator_kujira::state::Config, mantaswap::msg::ExecuteMsg};

use crate::helpers::{
    eldorado_aggregator::EldoradoAggregatorExtension,
    suite::core::Project,
    suite::types::{ProjectAccount, ProjectCoin, ToAddress},
};

#[test]
fn swap_in_default() {
    let mut project = Project::new();

    let amount_in: u128 = 1_000;
    let denom_in = ProjectCoin::Shd;

    let amount_out: u128 = 420;
    let denom_out = ProjectCoin::Kuji;

    // SHD -> USK -> KUJI
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Usk.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    let vault_balance_before = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Bob.to_address(), denom_out.to_string())
        .unwrap();

    let res = project
        .eldorado_aggregator_try_swap_in(
            &ProjectAccount::Alice,
            &ProjectAccount::Bob,
            mantaswap_msg,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();

    let vault_balance_after = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Bob.to_address(), denom_out.to_string())
        .unwrap();

    let digest = &res.events.last().unwrap().attributes.last().unwrap().value;

    assert_that(digest).is_equal_to("420 ukuji bob".to_string());

    assert_that(&vault_balance_after.amount.u128())
        .is_equal_to(vault_balance_before.amount.u128() + amount_out);
}

#[test]
#[should_panic(expected = "No funds sent")]
fn swap_in_without_funds() {
    let mut project = Project::new();

    let denom_in = ProjectCoin::Shd;

    let amount_out: u128 = 420;
    let denom_out = ProjectCoin::Kuji;

    // SHD -> USK -> KUJI
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Usk.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    project
        .eldorado_aggregator_try_swap_in::<u128>(
            &ProjectAccount::Alice,
            &ProjectAccount::Bob,
            mantaswap_msg,
            &None,
            &None,
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "Wrong MantaSwap message type!")]
fn swap_in_wrong_mantaswap_msg_type() {
    let mut project = Project::new();

    let addr = &Some(ProjectAccount::Admin.to_string());

    let amount_in: u128 = 1_000;
    let denom_in = ProjectCoin::Shd;

    let mantaswap_msg = &ExecuteMsg::UpdateConfig {
        fee: Some(1),
        owner: addr.to_owned(),
        treasury: addr.to_owned(),
        blend_oracle_contract: addr.to_owned(),
    };

    project
        .eldorado_aggregator_try_swap_in(
            &ProjectAccount::Alice,
            &ProjectAccount::Bob,
            mantaswap_msg,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();
}

#[test]
fn swap_out_default() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Kuji;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Usk;

    // KUJI -> SHD -> USK
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Shd.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    let vault_balance_before = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Alice.to_address(), denom_out.to_string())
        .unwrap();

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            mantaswap_msg,
            &None,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();

    let vault_balance_after = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Alice.to_address(), denom_out.to_string())
        .unwrap();

    assert_that(&vault_balance_after.amount.u128())
        .is_equal_to(vault_balance_before.amount.u128() + amount_out);
}

#[test]
#[should_panic(expected = "Sender does not have access permissions!")]
fn swap_out_unauthorized() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Kuji;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Usk;

    // KUJI -> SHD -> USK
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Shd.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Alice,
            &ProjectAccount::Admin,
            mantaswap_msg,
            &None,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "Must send reserve token 'ukuji'")]
fn swap_out_wrong_denom_in() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Shd;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Usk;

    // SHD -> KUJI -> USK
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Kuji.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            mantaswap_msg,
            &None,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();
}

#[test]
fn swap_out_ibc_token_without_ibc_channel() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Kuji;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Shd;

    // KUJI -> USK -> SHD
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Usk.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    let vault_balance_before = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Alice.to_address(), denom_out.to_string())
        .unwrap();

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            mantaswap_msg,
            &None,
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();

    let vault_balance_after = project
        .app
        .wrap()
        .query_balance(ProjectAccount::Alice.to_address(), denom_out.to_string())
        .unwrap();

    assert_that(&vault_balance_after.amount.u128())
        .is_equal_to(vault_balance_before.amount.u128() + amount_out);
}

#[test]
#[should_panic(expected = "The asset is not IBC token!")]
fn swap_out_ibc_channel_without_ibc_token() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Kuji;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Usk;

    // KUJI -> SHD -> USK
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Shd.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            mantaswap_msg,
            &Some("channel-0".to_string()),
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "Unexpected exec msg Transfer")]
fn swap_out_ibc_channel_with_ibc_token() {
    let mut project = Project::new();

    let amount_in: u128 = 420;
    let denom_in = ProjectCoin::Kuji;

    let amount_out: u128 = 1_000;
    let denom_out = ProjectCoin::Shd;

    // KUJI -> USK -> SHD
    let mantaswap_msg = &ExecuteMsg::Swap {
        recipient: None,
        min_return: Some(vec![coin(amount_out, denom_out.to_string())]),
        stages: vec![
            vec![(
                Addr::unchecked("contract91"),
                Denom::from(ProjectCoin::Usk.to_string()),
            )],
            vec![(
                Addr::unchecked("contract90"),
                Denom::from(denom_in.to_string()),
            )],
        ],
    };

    project
        .eldorado_aggregator_try_swap_out(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            mantaswap_msg,
            &Some("channel-0".to_string()),
            &Some(amount_in),
            &Some(denom_in),
        )
        .unwrap();
}

#[test]
fn update_vault_default() {
    let mut project = Project::new();

    let config_before = project.eldorado_aggregator_query_config().unwrap();

    project
        .eldorado_aggregator_try_update_vault::<u128>(
            &ProjectAccount::Owner,
            &ProjectAccount::Alice,
            &None,
            &None,
        )
        .unwrap();

    let config_after = project.eldorado_aggregator_query_config().unwrap();

    assert_that(&config_before.vault).is_equal_to(&ProjectAccount::Bob.to_address());
    assert_that(&config_after.vault).is_equal_to(&ProjectAccount::Alice.to_address());
}

#[test]
fn update_vault_by_admin() {
    let mut project = Project::new();

    let config_before = project.eldorado_aggregator_query_config().unwrap();

    project
        .eldorado_aggregator_try_update_vault::<u128>(
            &ProjectAccount::Admin,
            &ProjectAccount::Alice,
            &None,
            &None,
        )
        .unwrap();

    let config_after = project.eldorado_aggregator_query_config().unwrap();

    assert_that(&config_before.vault).is_equal_to(&ProjectAccount::Bob.to_address());
    assert_that(&config_after.vault).is_equal_to(&ProjectAccount::Alice.to_address());
}

#[test]
#[should_panic(expected = "Sender does not have access permissions!")]
fn update_vault_unauthorized() {
    let mut project = Project::new();

    project
        .eldorado_aggregator_try_update_vault::<u128>(
            &ProjectAccount::Bob,
            &ProjectAccount::Alice,
            &None,
            &None,
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "This message does no accept funds")]
fn update_vault_with_funds() {
    let mut project = Project::new();

    project
        .eldorado_aggregator_try_update_vault::<u128>(
            &ProjectAccount::Owner,
            &ProjectAccount::Alice,
            &Some(1_000),
            &Some(ProjectCoin::Kuji),
        )
        .unwrap();
}

#[test]
fn update_config_default() {
    let mut project = Project::new();

    let config_before = project.eldorado_aggregator_query_config().unwrap();

    project
        .eldorado_aggregator_try_update_config::<u128>(
            &ProjectAccount::Admin,
            &Some(ProjectAccount::Bob),
            &Some(ProjectAccount::Alice),
            &Some(5),
            &Some(&ProjectAccount::Owner),
            &None,
            &None,
        )
        .unwrap();

    let config_after = project.eldorado_aggregator_query_config().unwrap();

    assert_that(&config_before).is_equal_to(&Config {
        admin: ProjectAccount::Admin.to_address(),
        owner: ProjectAccount::Owner.to_address(),
        router: project.get_mantaswap_router_address(),
        vault: ProjectAccount::Bob.to_address(),
        ibc_timeout: 15 * 60,
    });

    assert_that(&config_after).is_equal_to(&Config {
        admin: ProjectAccount::Admin.to_address(),
        owner: ProjectAccount::Bob.to_address(),
        router: ProjectAccount::Owner.to_address(),
        vault: ProjectAccount::Alice.to_address(),
        ibc_timeout: 5 * 60,
    });
}

#[test]
#[should_panic(expected = "Sender does not have access permissions!")]
fn update_config_unauthorized() {
    let mut project = Project::new();

    project
        .eldorado_aggregator_try_update_config::<u128>(
            &ProjectAccount::Alice,
            &Some(ProjectAccount::Bob),
            &Some(ProjectAccount::Alice),
            &Some(5),
            &Some(&ProjectAccount::Owner),
            &None,
            &None,
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "This message does no accept funds")]
fn update_config_with_funds() {
    let mut project = Project::new();

    project
        .eldorado_aggregator_try_update_config::<u128>(
            &ProjectAccount::Admin,
            &Some(ProjectAccount::Bob),
            &Some(ProjectAccount::Alice),
            &Some(5),
            &Some(&ProjectAccount::Owner),
            &Some(1_000),
            &Some(ProjectCoin::Kuji),
        )
        .unwrap();
}
