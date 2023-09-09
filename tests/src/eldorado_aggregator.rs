use cosmwasm_std::{coin, Addr};

use speculoos::assert_that;

use kujira::Denom;

use eldorado_base::mantaswap::msg::ExecuteMsg::Swap;

use crate::helpers::{
    eldorado_aggregator::EldoradoAggregatorExtension,
    suite::core::Project,
    suite::types::{ProjectAccount, ProjectCoin, ToAddress},
};

#[test]
fn swap_in_default() {
    let mut project = Project::new();

    let amount_in: u128 = 1_000;
    let denom_in = &ProjectCoin::Shd;

    let amount_out: u128 = 420;
    let denom_out = &ProjectCoin::Kuji;

    // SHD -> USK -> KUJI
    let mantaswap_msg = &Swap {
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
        .query_balance(
            ProjectAccount::Bob.to_address(),
            ProjectCoin::Kuji.to_string(),
        )
        .unwrap();

    let res = project
        .eldorado_aggregator_try_swap_in(
            &ProjectAccount::Alice,
            &ProjectAccount::Bob,
            mantaswap_msg,
            amount_in,
            denom_in,
        )
        .unwrap();

    let vault_balance_after = project
        .app
        .wrap()
        .query_balance(
            ProjectAccount::Bob.to_address(),
            ProjectCoin::Kuji.to_string(),
        )
        .unwrap();

    println!("{:#?}", res);

    assert_that(&vault_balance_after.amount.u128())
        .is_equal_to(&(vault_balance_before.amount.u128() + amount_out));
}
